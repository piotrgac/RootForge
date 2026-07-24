use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Linux,
    System,
    Network,
    Security,
    Shell,
    #[serde(rename = "devops")]
    DevOps,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Linux => "Linux",
            Category::System => "System",
            Category::Network => "Sieć",
            Category::Security => "Bezpieczeństwo",
            Category::Shell => "Shell",
            Category::DevOps => "DevOps",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Challenge {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub category: Category,
    pub difficulty: u8,
    pub completed: bool,
    #[serde(default)]
    pub details: Option<String>,
    #[serde(default)]
    pub stage: u8,
    #[serde(default)]
    pub exam_tag: Option<String>,
    #[serde(default)]
    pub depends_on: Vec<u32>,
    #[serde(default)]
    pub last_reviewed: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Troubleshoot {
    pub id: u32,
    pub title: String,
    pub scenario: String,
    pub category: Category,
    pub difficulty: u8,
    pub hints: Vec<String>,
    pub solution: String,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub challenge_ids: Vec<u32>,
    pub unlocked: bool,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub guide: String,
    pub github_repo: Option<String>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: u32,
    pub question: String,
    pub options: Vec<String>,
    pub correct_index: usize,
    pub category: Category,
    pub explanation: String,
    #[serde(default)]
    pub stage: u8,
    #[serde(default)]
    pub hint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizResult {
    pub quiz_id: u32,
    pub correct: bool,
    #[serde(default)]
    pub confidence: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mission {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub steps: Vec<u32>,
    pub completed: bool,
    pub xp_reward: u32,
    #[serde(default)]
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StudySession {
    pub date: String,
    pub duration_minutes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrongAnswer {
    pub quiz_id: u32,
    pub wrong_count: u32,
    pub last_wrong: String,
    pub next_review: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExamAttempt {
    pub date: String,
    pub score: u32,
    pub total: u32,
    pub passed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub unlocked: bool,
    #[serde(default)]
    pub unlocked_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeedRecord {
    pub command_id: u32,
    pub time_seconds: u32,
    pub correct: bool,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub xp: u32,
    pub level: u32,
    pub challenges: Vec<Challenge>,
    pub milestones: Vec<Milestone>,
    pub projects: Vec<Project>,
    pub quizzes: Vec<Quiz>,
    pub quiz_results: Vec<QuizResult>,
    pub github_token: Option<String>,
    pub github_gist_id: Option<String>,
    pub sessions: Vec<StudySession>,
    pub daily_goal_minutes: u32,
    pub wrong_answers: Vec<WrongAnswer>,
    pub exam_attempts: Vec<ExamAttempt>,
    pub achievements: Vec<Achievement>,
    pub speed_records: Vec<SpeedRecord>,
    pub missions: Vec<Mission>,
    pub last_daily_date: Option<String>,
    pub daily_streak: u32,
    pub troubleshoot: Vec<Troubleshoot>,
    pub troubleshoot_results: Vec<u32>,
}

impl AppData {
    pub fn default_data() -> Self {
        let challenges: Vec<Challenge> = serde_json::from_str(include_str!("../data/challenges.json"))
            .expect("Failed to parse challenges.json");
        let milestones: Vec<Milestone> = serde_json::from_str(include_str!("../data/milestones.json"))
            .expect("Failed to parse milestones.json");
        let projects: Vec<Project> = serde_json::from_str(include_str!("../data/projects.json"))
            .expect("Failed to parse projects.json");
        let quizzes: Vec<Quiz> = serde_json::from_str(include_str!("../data/quiz.json"))
            .expect("Failed to parse quiz.json");
        let achievements: Vec<Achievement> = serde_json::from_str(include_str!("../data/achievements.json"))
            .expect("Failed to parse achievements.json");
        let missions: Vec<Mission> = serde_json::from_str(include_str!("../data/missions.json"))
            .expect("Failed to parse missions.json");
        let troubleshoot: Vec<Troubleshoot> = serde_json::from_str(include_str!("../data/troubleshoot.json"))
            .expect("Failed to parse troubleshoot.json");

        AppData {
            xp: 0,
            level: 1,
            challenges,
            milestones,
            projects,
            quizzes,
            quiz_results: Vec::new(),
            github_token: None,
            github_gist_id: None,
            sessions: Vec::new(),
            daily_goal_minutes: 30,
            wrong_answers: Vec::new(),
            exam_attempts: Vec::new(),
            achievements,
            speed_records: Vec::new(),
            missions,
            last_daily_date: None,
            daily_streak: 0,
            troubleshoot,
            troubleshoot_results: Vec::new(),
        }
    }
}
pub struct DataStore {
    pub data: Mutex<AppData>,
    path: PathBuf,
}

impl DataStore {
    pub fn new(app_dir: PathBuf) -> Self {
        fs::create_dir_all(&app_dir).expect("Failed to create app data directory");
        let path = app_dir.join("data.json");
        let data = Self::load_or_default(&path);
        DataStore { data: Mutex::new(data), path }
    }

    fn load_or_default(path: &PathBuf) -> AppData {
        if path.exists() {
            let content = fs::read_to_string(path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_else(|_| AppData::default_data())
        } else {
            AppData::default_data()
        }
    }

    pub fn save(&self) {
        if let Ok(data) = self.data.lock() {
            if let Ok(json) = serde_json::to_string_pretty(&*data) {
                fs::write(&self.path, json).ok();
            }
        }
    }

    pub fn get_json(&self) -> String {
        self.data.lock()
            .ok()
            .and_then(|d| serde_json::to_string_pretty(&*d).ok())
            .unwrap_or_default()
    }

    pub fn load_json(&self, json: &str) -> Result<(), String> {
        let new_data: AppData = serde_json::from_str(json).map_err(|e| e.to_string())?;
        let mut data = self.data.lock().map_err(|e| e.to_string())?;
        *data = new_data;
        drop(data);
        self.save();
        Ok(())
    }

    pub fn save_github_token(&self, token: String) {
        let mut data = self.data.lock().unwrap();
        data.github_token = Some(token);
        drop(data);
        self.save();
    }

    pub fn save_daily_goal(&self, minutes: u32) {
        let mut data = self.data.lock().unwrap();
        data.daily_goal_minutes = minutes;
        drop(data);
        self.save();
    }

    pub fn log_session(&self, minutes: u32) {
        let mut data = self.data.lock().unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        data.sessions.push(StudySession { date: today, duration_minutes: minutes });
        drop(data);
        self.save();
    }

    pub fn complete_challenge(&self, challenge_id: u32) -> (bool, u32, u32) {
        let mut data = self.data.lock().unwrap();
        let completed_ids: std::collections::HashSet<u32> = data.challenges.iter().filter(|c| c.completed).map(|c| c.id).collect();
        // Build the new set including the one we're about to complete
        let new_completed_ids: std::collections::HashSet<u32> = {
            let mut set = completed_ids.clone();
            set.insert(challenge_id);
            set
        };
        if let Some(ch) = data.challenges.iter_mut().find(|c| c.id == challenge_id) {
            if !ch.completed {
                ch.completed = true;
                ch.last_reviewed = Some(chrono::Local::now().format("%Y-%m-%d").to_string());
                let xp_gain = ch.difficulty as u32 * 10;
                data.xp += xp_gain;
                let new_level = 1 + data.xp / 100;
                data.level = new_level;
                // Check milestones against new completed set
                for m in data.milestones.iter_mut() {
                    if !m.completed && m.challenge_ids.iter().all(|cid| new_completed_ids.contains(cid)) {
                        m.completed = true;
                    }
                }
                let _ = self.check_achievements_inner(&mut data);
                drop(data);
                self.save();
                return (true, xp_gain, new_level);
            }
        }
        (false, 0, 1)
    }

    pub fn update_github_repo(&self, project_id: u32, repo: String) -> bool {
        let mut data = self.data.lock().unwrap();
        if let Some(p) = data.projects.iter_mut().find(|p| p.id == project_id) {
            p.github_repo = Some(repo.clone());
            drop(data);
            self.save();
            return true;
        }
        false
    }

    pub fn complete_project(&self, project_id: u32) -> (bool, u32) {
        let mut data = self.data.lock().unwrap();
        if let Some(p) = data.projects.iter_mut().find(|p| p.id == project_id) {
            if !p.completed {
                p.completed = true;
                data.xp += 50;
                let new_level = 1 + data.xp / 100;
                data.level = new_level;
                let _ = self.check_achievements_inner(&mut data);
                drop(data);
                self.save();
                return (true, new_level);
            }
        }
        (false, 1)
    }

    pub fn submit_quiz_answer(&self, quiz_id: u32, answer_index: usize, confidence: Option<u32>) -> (bool, bool, String) {
        let mut data = self.data.lock().unwrap();
        let q_idx = data.quizzes.iter().position(|q| q.id == quiz_id);
        match q_idx {
            Some(idx) => {
                let q = &data.quizzes[idx];
                let correct = answer_index == q.correct_index;
                let explanation = q.explanation.clone();
                data.quiz_results.push(QuizResult { quiz_id, correct, confidence });
                if correct {
                    data.xp += 15;
                    data.level = 1 + data.xp / 100;
                    // correct → remove from wrong_answers tracking
                    data.wrong_answers.retain(|w| w.quiz_id != quiz_id);
                } else {
                    // wrong → spaced repetition tracking
                    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                    let intervals = [0, 1, 3, 7, 14];
                    if let Some(wa) = data.wrong_answers.iter_mut().find(|w| w.quiz_id == quiz_id) {
                        wa.wrong_count = wa.wrong_count.saturating_add(1).min(99);
                        wa.last_wrong = today.clone();
                        let idx = (wa.wrong_count as usize).min(intervals.len() - 1);
                        let offset = intervals[idx];
                        let next = chrono::Local::now() + chrono::Duration::days(offset as i64);
                        wa.next_review = next.format("%Y-%m-%d").to_string();
                    } else {
                        let next = chrono::Local::now() + chrono::Duration::days(0);
                        data.wrong_answers.push(WrongAnswer {
                            quiz_id,
                            wrong_count: 1,
                            last_wrong: today.clone(),
                            next_review: next.format("%Y-%m-%d").to_string(),
                        });
                    }
                }
                let _ = self.check_achievements_inner(&mut data);
                drop(data);
                self.save();
                (true, correct, explanation)
            }
            None => (false, false, String::new()),
        }
    }

    pub fn mark_quiz_correct_in_review(&self, quiz_id: u32) {
        let mut data = self.data.lock().unwrap();
        data.wrong_answers.retain(|w| w.quiz_id != quiz_id);
        data.xp += 5;
        data.level = 1 + data.xp / 100;
        let _ = self.check_achievements_inner(&mut data);
        drop(data);
        self.save();
    }

    pub fn finish_exam(&self, score: u32, total: u32) {
        let mut data = self.data.lock().unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let passed = score as f64 / total as f64 >= 0.7;
        data.exam_attempts.push(ExamAttempt { date: today, score, total, passed });
        if passed {
            data.xp += 30;
            data.level = 1 + data.xp / 100;
        }
        let _ = self.check_achievements_inner(&mut data);
        drop(data);
        self.save();
    }

    pub fn claim_daily(&self) -> (String, u32, u32) {
        let mut data = self.data.lock().unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let xp_reward = 20 + data.daily_streak * 5;

        if data.last_daily_date.as_deref() == Some(&today) {
            let streak = data.daily_streak;
            drop(data);
            return ("already_claimed".into(), streak, xp_reward);
        }

        // Check if yesterday was claimed (or this is first claim)
        let yesterday = (chrono::Local::now() - chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
        if data.last_daily_date.as_deref() == Some(&yesterday) || data.last_daily_date.is_none() {
            data.daily_streak += 1;
        } else {
            data.daily_streak = 1; // reset streak
        }
        data.last_daily_date = Some(today.clone());
        data.xp += xp_reward;
        data.level = 1 + data.xp / 100;
        let streak = data.daily_streak;
        let _ = self.check_achievements_inner(&mut data);
        drop(data);
        self.save();
        ("claimed".into(), streak, xp_reward)
    }

    pub fn complete_mission_step(&self, mission_id: u32) -> (bool, Vec<u32>) {
        let mut data = self.data.lock().unwrap();
        if let Some(m) = data.missions.iter_mut().find(|m| m.id == mission_id) {
            if !m.completed {
                m.completed = true;
                data.xp += m.xp_reward;
                data.level = 1 + data.xp / 100;
                let unlocked = self.check_achievements_inner(&mut data);
                drop(data);
                self.save();
                return (true, unlocked);
            }
        }
        (false, vec![])
    }

    pub fn complete_troubleshoot(&self, id: u32) -> (bool, u32, Vec<u32>) {
        let mut data = self.data.lock().unwrap();
        if data.troubleshoot_results.contains(&id) {
            return (false, 0, vec![]);
        }
        data.troubleshoot_results.push(id);
        let xp_gain = 25;
        data.xp += xp_gain;
        data.level = 1 + data.xp / 100;
        let unlocked = self.check_achievements_inner(&mut data);
        drop(data);
        self.save();
        (true, xp_gain, unlocked)
    }

    pub fn review_challenge(&self, challenge_id: u32) {
        let mut data = self.data.lock().unwrap();
        if let Some(ch) = data.challenges.iter_mut().find(|c| c.id == challenge_id) {
            ch.last_reviewed = Some(chrono::Local::now().format("%Y-%m-%d").to_string());
        }
        drop(data);
        self.save();
    }

    pub fn reset_progress(&self) {
        let mut data = self.data.lock().unwrap();
        let dir = self.path.parent().unwrap_or(std::path::Path::new("."));
        let fresh = DataStore::new(dir.to_path_buf());
        if let Ok(fresh_data) = fresh.data.lock() {
            *data = fresh_data.clone();
        }
        drop(data);
        self.save();
    }

    pub fn finish_speed_challenge(&self, command_id: u32, time_seconds: u32, correct: bool) -> (u32, Vec<u32>) {
        let mut data = self.data.lock().unwrap();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        data.speed_records.push(SpeedRecord { command_id, time_seconds, correct, date: today.clone() });
        let mut xp_gain = 0;
        if correct {
            xp_gain = if time_seconds <= 5 { 20 } else if time_seconds <= 15 { 15 } else if time_seconds <= 30 { 10 } else { 5 };
            data.xp += xp_gain;
            data.level = 1 + data.xp / 100;
        }
        let unlocked = self.check_achievements_inner(&mut data);
        drop(data);
        self.save();
        (xp_gain, unlocked)
    }

    pub fn check_achievements(&self) -> Vec<u32> {
        let mut data = self.data.lock().unwrap();
        let unlocked = self.check_achievements_inner(&mut data);
        drop(data);
        self.save();
        unlocked
    }

    fn check_achievements_inner(&self, data: &mut AppData) -> Vec<u32> {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let mut newly_unlocked = Vec::new();

        for ach in data.achievements.iter_mut() {
            if ach.unlocked { continue; }
            let should_unlock = match ach.id {
                1 => data.challenges.iter().filter(|c| c.completed).count() >= 1,
                2 => data.challenges.iter().filter(|c| c.completed).count() >= 10,
                3 => data.challenges.iter().filter(|c| c.completed).count() >= 36,
                4 => data.challenges.iter().all(|c| c.completed),
                5 => data.projects.iter().any(|p| p.completed),
                6 => data.projects.iter().all(|p| p.completed),
                7 => data.quiz_results.len() >= 1 && data.quiz_results.iter().all(|r| r.correct),
                8 => {
                    let days: std::collections::BTreeSet<String> = data.sessions.iter().map(|s| s.date.clone()).collect();
                    if days.len() < 7 { false } else {
                        let mut sorted: Vec<_> = days.into_iter().collect();
                        sorted.sort();
                        sorted.last().map(|last| last.as_str() == today.as_str()).unwrap_or(false)
                            && sorted.len() >= 7
                    }
                },
                9 => data.quiz_results.iter().filter(|r| r.correct).count() >= 50,
                10 => data.xp >= 1000,
                11 => data.level >= 5,
                12 => data.speed_records.iter().any(|r| r.correct && r.time_seconds <= 5),
                13 => data.speed_records.iter().filter(|r| r.correct).count() >= 20,
                14 => data.exam_attempts.iter().any(|e| e.passed),
                15 => {
                    let total_minutes: u32 = data.sessions.iter().map(|s| s.duration_minutes).sum();
                    total_minutes >= 600
                },
                16 => data.speed_records.iter().filter(|r| r.correct && r.command_id >= 1000 && r.command_id <= 1100).count() >= 5,
                17 => data.speed_records.iter().filter(|r| r.correct && r.command_id >= 1000 && r.command_id <= 1100).count() >= 15,
                18 => data.speed_records.iter().filter(|r| r.correct && r.command_id >= 1000 && r.command_id <= 1100).count() >= 30,
                19 => data.challenges.iter().filter(|c| c.completed && c.exam_tag.as_deref() == Some("devops")).count() >= 5,
                20 => data.challenges.iter().filter(|c| c.completed && c.exam_tag.as_deref() == Some("devops")).count() >= 10,
                21 => data.challenges.iter().filter(|c| c.completed && c.exam_tag.as_deref() == Some("devops")).count() >= 10,
                22 => data.challenges.iter().filter(|c| c.completed && c.exam_tag.as_deref() == Some("devops")).count() >= 10,
                23 => data.challenges.iter().filter(|c| c.completed && c.exam_tag.as_deref() == Some("devops")).count() >= 30,
                _ => false,
            };
            if should_unlock {
                ach.unlocked = true;
                ach.unlocked_at = Some(today.clone());
                newly_unlocked.push(ach.id);
            }
        }
        newly_unlocked
    }

    pub fn get_unlocked_achievements(&self) -> Vec<Achievement> {
        let data = self.data.lock().unwrap();
        data.achievements.iter().filter(|a| a.unlocked).cloned().collect()
    }

    pub fn export_progress_markdown(&self) -> String {
        let data = self.data.lock().unwrap();
        let completed = data.challenges.iter().filter(|c| c.completed).count();
        let total = data.challenges.len();
        let projects_done = data.projects.iter().filter(|p| p.completed).count();
        let projects_total = data.projects.len();
        let quiz_correct = data.quiz_results.iter().filter(|r| r.correct).count();
        let quiz_total = data.quiz_results.len();
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();

        let mut md = String::new();
        md.push_str(&format!("# RootForge – Progress Report\n\n"));
        md.push_str(&format!("**Date:** {}  \n", today));
        md.push_str(&format!("**Level:** {}  \n", data.level));
        md.push_str(&format!("**XP:** {}  \n\n", data.xp));

        md.push_str("## Overview\n\n");
        md.push_str(&format!("- Challenges: {}/{} ({:.0}%)\n", completed, total, completed as f64 / total as f64 * 100.0));
        md.push_str(&format!("- Projects: {}/{}\n", projects_done, projects_total));
        md.push_str(&format!("- Quiz answers: {}/{}\n\n", quiz_correct, quiz_total));

        md.push_str("## Completed Challenges\n\n");
        for ch in data.challenges.iter().filter(|c| c.completed) {
            md.push_str(&format!("- [x] **{}** ({}, difficulty: {})\n", ch.title, ch.category.as_str(), ch.difficulty));
        }

        md.push_str("\n## Completed Projects\n\n");
        for p in data.projects.iter().filter(|p| p.completed) {
            md.push_str(&format!("- [x] **{}**", p.title));
            if let Some(ref repo) = p.github_repo {
                md.push_str(&format!(" – [GitHub]({})", repo));
            }
            md.push_str("\n");
        }

        if quiz_correct > 0 {
            md.push_str("\n## Quiz Results\n\n");
            for r in data.quiz_results.iter().filter(|r| r.correct) {
                if let Some(q) = data.quizzes.iter().find(|q| q.id == r.quiz_id) {
                    md.push_str(&format!("- ✅ {}\n", q.question));
                }
            }
        }

        md.push_str("\n---\n*Generated by RootForge*\n");
        md
    }
}

fn new_agent() -> ureq::Agent {
    let config = ureq::config::Config::builder()
        .timeout_connect(Some(std::time::Duration::from_secs(10)))
        .timeout_recv_body(Some(std::time::Duration::from_secs(30)))
        .build();
    ureq::Agent::new_with_config(config)
}

fn read_body_text(response: ureq::http::Response<ureq::Body>) -> Result<String, String> {
    let mut body = response.into_body();
    body.read_to_string().map_err(|e| format!("Body read error: {}", e))
}

pub fn github_save_gist(token: &str, gist_id: &Option<String>, filename: &str, content: &str) -> Result<String, String> {
    let json_body = serde_json::json!({
        "description": "RootForge – Linux SysAdmin progress backup",
        "public": false,
        "files": {
            filename: { "content": content }
        }
    });

    let agent = new_agent();
    let url = match gist_id {
        Some(id) => format!("https://api.github.com/gists/{}", id),
        None => "https://api.github.com/gists".to_string(),
    };

    let response = if gist_id.is_some() {
        agent.patch(&url)
    } else {
        agent.post(&url)
    }
    .header("Authorization", &format!("Bearer {}", token))
    .header("User-Agent", "RootForge/1.0")
    .header("Accept", "application/vnd.github.v3+json")
    .send_json(json_body)
    .map_err(|e| format!("GitHub API error: {}", e))?;

    let body_text = read_body_text(response)?;
    let json: serde_json::Value = serde_json::from_str(&body_text)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let new_id = json["id"].as_str()
        .ok_or_else(|| "Failed to get gist ID from response".to_string())?
        .to_string();

    Ok(new_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> DataStore {
        let dir = std::env::temp_dir().join(format!("rootforge_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        DataStore::new(dir)
    }

    #[test]
    fn test_complete_challenge_awards_xp() {
        let ds = setup();
        let (success, xp, level) = ds.complete_challenge(1);
        assert!(success, "Challenge 1 should be completable");
        assert_eq!(xp, 10, "Difficulty 1 * 10 = 10 XP");
        assert_eq!(level, 1, "10 XP = level 1");
    }

    #[test]
    fn test_complete_challenge_idempotent() {
        let ds = setup();
        ds.complete_challenge(1);
        let (success, xp, _) = ds.complete_challenge(1);
        assert!(!success, "Second completion should return false");
        assert_eq!(xp, 0, "No XP for duplicate");
    }

    #[test]
    fn test_complete_challenge_level_up() {
        let ds = setup();
        // Complete difficulty 4 challenge = 40 XP, need 100 for level 2
        // Complete enough challenges to exceed 100 XP
        for id in [10, 11, 12, 13].iter() {
            ds.complete_challenge(*id);
        }
        let data = ds.data.lock().unwrap();
        assert!(data.xp >= 100, "XP should be 100+ got {}", data.xp);
        assert_eq!(data.level, 1 + data.xp / 100, "Level should match XP/100");
    }

    #[test]
    fn test_complete_project() {
        let ds = setup();
        let (success, level) = ds.complete_project(1);
        assert!(success);
        assert_eq!(level, 1, "50 XP = level 1");
        // duplicate
        let (success2, _) = ds.complete_project(1);
        assert!(!success2);
    }

    #[test]
    fn test_submit_quiz_correct() {
        let ds = setup();
        // Quiz 1: "Podstawy terminala" question - let's find correct answer
        // Just run it with index matching the correct one
        // Quiz[0] has correct_index = some value
        let q_id = { ds.data.lock().unwrap().quizzes[0].id };
        let correct_idx = { let data = ds.data.lock().unwrap(); data.quizzes.iter().find(|q| q.id == q_id).unwrap().correct_index };

        let (found, correct, _) = ds.submit_quiz_answer(q_id, correct_idx, None);
        assert!(found);
        assert!(correct);

        let data = ds.data.lock().unwrap();
        assert!(data.quiz_results.iter().any(|r| r.quiz_id == q_id && r.correct));
        assert_eq!(data.xp, 15);
    }

    #[test]
    fn test_submit_quiz_wrong_tracks_spaced_repetition() {
        let ds = setup();
        let q_id = ds.data.lock().unwrap().quizzes[0].id;
        let correct_idx = { let d = ds.data.lock().unwrap(); d.quizzes.iter().find(|q| q.id == q_id).unwrap().correct_index };
        let wrong_idx = if correct_idx == 0 { 1 } else { 0 };

        ds.submit_quiz_answer(q_id, wrong_idx, None);

        let data = ds.data.lock().unwrap();
        let wa = data.wrong_answers.iter().find(|w| w.quiz_id == q_id);
        assert!(wa.is_some(), "Wrong answer should be tracked");
        assert_eq!(wa.unwrap().wrong_count, 1);
    }

    #[test]
    fn test_mark_quiz_correct_in_review() {
        let ds = setup();
        let q_id = ds.data.lock().unwrap().quizzes[0].id;
        let correct_idx = { let d = ds.data.lock().unwrap(); d.quizzes.iter().find(|q| q.id == q_id).unwrap().correct_index };
        let wrong_idx = 0.max(if correct_idx == 0 { 1 } else { 0 });

        ds.submit_quiz_answer(q_id, wrong_idx, None);
        let before = { ds.data.lock().unwrap().xp };
        ds.mark_quiz_correct_in_review(q_id);

        let data = ds.data.lock().unwrap();
        assert!(data.wrong_answers.iter().all(|w| w.quiz_id != q_id), "Removed from wrong_answers");
        assert_eq!(data.xp, before + 5, "Review grants +5 XP");
    }

    #[test]
    fn test_finish_exam_pass() {
        let ds = setup();
        ds.finish_exam(11, 15); // 73% > 70%
        let data = ds.data.lock().unwrap();
        assert!(data.exam_attempts[0].passed);
        assert_eq!(data.xp, 30);
    }

    #[test]
    fn test_finish_exam_fail() {
        let ds = setup();
        ds.finish_exam(5, 15); // 33% < 70%
        let data = ds.data.lock().unwrap();
        assert!(!data.exam_attempts[0].passed);
        assert_eq!(data.xp, 0);
    }

    #[test]
    fn test_export_markdown_contains_sections() {
        let ds = setup();
        ds.complete_challenge(1);
        ds.complete_project(1);
        let md = ds.export_progress_markdown();
        assert!(md.contains("# RootForge – Progress Report"));
        assert!(md.contains("Level:"));
        assert!(md.contains("Challenges: 1/"));
        assert!(md.contains("*Generated by RootForge*"));
    }

    #[test]
    fn test_daily_goal() {
        let ds = setup();
        ds.save_daily_goal(45);
        let data = ds.data.lock().unwrap();
        assert_eq!(data.daily_goal_minutes, 45);
    }

    #[test]
    fn test_log_study_session() {
        let ds = setup();
        ds.log_session(30);
        let data = ds.data.lock().unwrap();
        assert_eq!(data.sessions.len(), 1);
        assert_eq!(data.sessions[0].duration_minutes, 30);
    }

    #[test]
    fn test_update_github_repo() {
        let ds = setup();
        let ok = ds.update_github_repo(1, "https://github.com/user/repo".into());
        assert!(ok);
        let data = ds.data.lock().unwrap();
        let p = data.projects.iter().find(|p| p.id == 1).unwrap();
        assert_eq!(p.github_repo.as_deref(), Some("https://github.com/user/repo"));
    }

    #[test]
    fn test_save_and_load_json() {
        let dir = std::env::temp_dir().join(format!("rootforge_saveload_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        let ds = DataStore::new(dir.clone());

        ds.complete_challenge(1);
        let json = ds.get_json();
        assert!(json.contains("\"completed\": true"));

        // Create new DataStore pointing to same file to verify persistence
        let ds2 = DataStore::new(dir.clone());
        let data2 = ds2.data.lock().unwrap();
        assert_eq!(data2.xp, 10);
        let _ = fs::remove_dir_all(&dir);
    }
}

pub fn github_load_gist(token: &str, gist_id: &str) -> Result<String, String> {
    let agent = new_agent();
    let url = format!("https://api.github.com/gists/{}", gist_id);

    let response = agent.get(&url)
        .header("Authorization", &format!("Bearer {}", token))
        .header("User-Agent", "RootForge/1.0")
        .header("Accept", "application/vnd.github.v3+json")
        .call()
        .map_err(|e| format!("GitHub API error: {}", e))?;

    let body_text = read_body_text(response)?;
    let json: serde_json::Value = serde_json::from_str(&body_text)
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let files = json["files"].as_object()
        .ok_or("No files in gist")?;

    let content = files.values()
        .next()
        .and_then(|f| f["content"].as_str())
        .ok_or("No content in gist file")?
        .to_string();

    Ok(content)
}
