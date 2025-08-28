use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct QuestionSet {
    metadata: Metadata,
    questions: Vec<Question>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    title: String,
    description: String,
    mode: String,
    #[serde(rename = "targetAge")]
    target_age: String,
    subject: String,
}

#[derive(Debug, Deserialize)]
struct Question {
    id: String,
    question: String,
    image1: Option<String>,
    image2: Option<String>,
    choices: Vec<String>,
    #[serde(rename = "correctAnswer")]
    correct_answer: usize,
    explanation: String,
    category: String,
    difficulty: String,
}

fn main() -> Result<()> {
    println!("🔍 Rendering All Ross Sea Questions");
    println!("====================================\n");

    let data_dir = Path::new("data");
    let entries = fs::read_dir(data_dir)?;

    let mut total_questions = 0;
    let mut question_sets_found = 0;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            let filename = path.file_name().unwrap().to_string_lossy();
            
            if filename.starts_with("questions_ross_sea") {
                println!("📁 Processing: {}", filename);
                
                let content = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read {}", path.display()))?;
                
                let question_set: QuestionSet = serde_json::from_str(&content)
                    .with_context(|| format!("Failed to parse {}", path.display()))?;
                
                println!("   📚 Title: {}", question_set.metadata.title);
                println!("   🎯 Mode: {}", question_set.metadata.mode);
                println!("   👥 Target Age: {}", question_set.metadata.target_age);
                println!("   📊 Questions: {}\n", question_set.questions.len());
                
                // Render each question
                for (idx, question) in question_set.questions.iter().enumerate() {
                    println!("   Question {} (ID: {}):", idx + 1, question.id);
                    println!("   ├─ Text: {}", question.question);
                    println!("   ├─ Category: {}", question.category);
                    println!("   ├─ Difficulty: {}", question.difficulty);
                    
                    if let Some(img1) = &question.image1 {
                        println!("   ├─ Question Image: {}", img1);
                    }
                    
                    if let Some(img2) = &question.image2 {
                        println!("   ├─ Answer Image: {}", img2);
                    }
                    
                    println!("   ├─ Choices:");
                    for (choice_idx, choice) in question.choices.iter().enumerate() {
                        let marker = if choice_idx == question.correct_answer { "✅" } else { "  " };
                        println!("   │  {} [{}] {}", marker, choice_idx, choice);
                    }
                    
                    println!("   └─ Explanation: {}\n", question.explanation);
                }
                
                total_questions += question_set.questions.len();
                question_sets_found += 1;
                
                println!("   ✅ Successfully rendered {} questions from {}\n", 
                         question_set.questions.len(), filename);
                println!("   {}", "─".repeat(60));
                println!();
            }
        }
    }
    
    println!("====================================");
    println!("📊 Summary:");
    println!("   Total question sets: {}", question_sets_found);
    println!("   Total questions: {}", total_questions);
    println!("   ✅ All questions successfully rendered!");
    
    // Validate image references
    println!("\n🖼️  Validating Image References:");
    let mut missing_images = Vec::new();
    
    // Re-read to check images
    for entry in fs::read_dir(data_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            let filename = path.file_name().unwrap().to_string_lossy();
            
            if filename.starts_with("questions_ross_sea") {
                let content = fs::read_to_string(&path)?;
                let question_set: QuestionSet = serde_json::from_str(&content)?;
                
                for question in &question_set.questions {
                    if let Some(img1) = &question.image1 {
                        if !Path::new(img1).exists() {
                            missing_images.push((question.id.clone(), img1.clone(), "image1"));
                        }
                    }
                    
                    if let Some(img2) = &question.image2 {
                        if !Path::new(img2).exists() {
                            missing_images.push((question.id.clone(), img2.clone(), "image2"));
                        }
                    }
                }
            }
        }
    }
    
    if missing_images.is_empty() {
        println!("   ✅ All image references are valid!");
    } else {
        println!("   ⚠️  Found {} missing image references:", missing_images.len());
        for (id, img, field) in missing_images {
            println!("      - Question {} {} references missing: {}", id, field, img);
        }
    }
    
    Ok(())
}