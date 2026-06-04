mod data;

use data::{AppData, DataStore};
use tauri::Manager;

struct AppState {
    store: DataStore,
}

#[tauri::command]
fn get_dashboard_stats(state: tauri::State<'_, AppState>) -> Result<AppData, String> {
    let data = state.store.data.lock().map_err(|e| e.to_string())?;
    Ok(data.clone())
}

#[tauri::command]
fn complete_challenge(state: tauri::State<'_, AppState>, id: u32) -> Result<(bool, u32, u32), String> {
    let (success, xp_gain, level) = state.store.complete_challenge(id);
    if success {
        let _ = notify_rust(&format!("Challenge ukończony! +{} XP", xp_gain));
    }
    Ok((success, xp_gain, level))
}

#[tauri::command]
fn update_project_repo(state: tauri::State<'_, AppState>, id: u32, repo: String) -> Result<bool, String> {
    Ok(state.store.update_github_repo(id, repo))
}

#[tauri::command]
fn complete_project(state: tauri::State<'_, AppState>, id: u32) -> Result<(bool, u32), String> {
    let (success, level) = state.store.complete_project(id);
    if success {
        let _ = notify_rust(&"Projekt ukończony! +50 XP");
    }
    Ok((success, level))
}

#[tauri::command]
fn submit_quiz(state: tauri::State<'_, AppState>, id: u32, answer: usize) -> Result<(bool, bool, String), String> {
    let (found, correct, explanation) = state.store.submit_quiz_answer(id, answer);
    if correct {
        let _ = notify_rust(&"Poprawna odpowiedź! +15 XP");
    }
    Ok((found, correct, explanation))
}

#[tauri::command]
fn get_review_questions(state: tauri::State<'_, AppState>) -> Result<Vec<data::WrongAnswer>, String> {
    let data = state.store.data.lock().map_err(|e| e.to_string())?;
    Ok(data.wrong_answers.clone())
}

#[tauri::command]
fn mark_quiz_correct_in_review(state: tauri::State<'_, AppState>, id: u32) -> Result<(), String> {
    state.store.mark_quiz_correct_in_review(id);
    Ok(())
}

#[tauri::command]
fn finish_exam(state: tauri::State<'_, AppState>, score: u32, total: u32) -> Result<(), String> {
    state.store.finish_exam(score, total);
    if score as f64 / total as f64 >= 0.7 {
        let _ = notify_rust(&format!("Egzamin zdany! Wynik: {}/{} (+30 XP)", score, total));
    } else {
        let _ = notify_rust(&format!("Egzamin niezdany. Wynik: {}/{} (potrzebne 70%)", score, total));
    }
    Ok(())
}

#[tauri::command]
fn send_test_notification() -> Result<(), String> {
    notify_rust("RootForge: Pora na dzienne wyzwanie!")
}

#[tauri::command]
fn save_github_token(state: tauri::State<'_, AppState>, token: String) -> Result<(), String> {
    state.store.save_github_token(token);
    Ok(())
}

#[tauri::command]
fn save_daily_goal(state: tauri::State<'_, AppState>, minutes: u32) -> Result<(), String> {
    state.store.save_daily_goal(minutes);
    Ok(())
}

#[tauri::command]
fn log_study_session(state: tauri::State<'_, AppState>, minutes: u32) -> Result<(), String> {
    state.store.log_session(minutes);
    Ok(())
}

#[tauri::command]
fn export_progress_markdown(state: tauri::State<'_, AppState>) -> Result<String, String> {
    Ok(state.store.export_progress_markdown())
}

#[tauri::command]
fn backup_to_github(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let token = {
        let data = state.store.data.lock().map_err(|e| e.to_string())?;
        data.github_token.clone().ok_or("Brak tokena GitHub. Skonfiguruj go w Ustawieniach.")?
    };
    let gist_id = {
        let data = state.store.data.lock().map_err(|e| e.to_string())?;
        data.github_gist_id.clone()
    };
    let json = state.store.get_json();
    let new_id = data::github_save_gist(&token, &gist_id, "rootforge-progress.json", &json)?;
    {
        let mut data = state.store.data.lock().map_err(|e| e.to_string())?;
        data.github_gist_id = Some(new_id.clone());
    }
    state.store.save();
    let _ = notify_rust("Backup zapisany na GitHub Gist!");
    Ok(new_id)
}

#[tauri::command]
fn restore_from_github(state: tauri::State<'_, AppState>) -> Result<String, String> {
    let (token, gist_id) = {
        let data = state.store.data.lock().map_err(|e| e.to_string())?;
        let token = data.github_token.clone().ok_or("Brak tokena GitHub")?;
        let gist_id = data.github_gist_id.clone().ok_or("Brak ID Gist. Najpierw zrób backup.")?;
        (token, gist_id)
    };
    let json = data::github_load_gist(&token, &gist_id)?;
    state.store.load_json(&json)?;
    let _ = notify_rust("Przywrócono dane z GitHub Gist!");
    Ok("Przywrócono pomyślnie".to_string())
}

fn notify_rust(message: &str) -> Result<(), String> {
    match notify_rust::Notification::new()
        .summary("RootForge")
        .body(message)
        .icon("dialog-information")
        .appname("RootForge")
        .timeout(notify_rust::Timeout::Milliseconds(5000))
        .show()
    {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Notification error: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir().unwrap_or_default();
            let store = DataStore::new(app_dir);
            app.manage(AppState { store });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_dashboard_stats,
            complete_challenge,
            update_project_repo,
            complete_project,
            submit_quiz,
            get_review_questions,
            mark_quiz_correct_in_review,
            finish_exam,
            send_test_notification,
            save_github_token,
            save_daily_goal,
            log_study_session,
            export_progress_markdown,
            backup_to_github,
            restore_from_github,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
