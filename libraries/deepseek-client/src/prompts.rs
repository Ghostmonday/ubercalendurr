use crate::models::ChatMessage;

pub struct PromptTemplates {
    extraction_system: String,
}

impl PromptTemplates {
    pub fn new() -> Self {
        Self {
            extraction_system: Self::build_extraction_system_prompt(),
        }
    }

    fn build_extraction_system_prompt() -> String {
        r#"You are a calendar assistant that extracts structured event information from natural language text.

## Output Format
You must output a JSON object:
{
    "event": "Event title",
    "date": "YYYY-MM-DD",
    "time": "HH:MM or null",
    "endTime": "HH:MM or null",
    "notes": "Additional details or null",
    "priority": "low|medium|high|urgent",
    "category": "work|personal|health|social|finance|education|other",
    "recurring": null or {...},
    "reminder": null or {...},
    "location": null or {...},
    "tags": [],
    "clarificationQuestions": []
}

## Rules
1. Extract event title - focus on the "what"
2. Dates in YYYY-MM-DD format:
   - "today" = current date
   - "tomorrow" = current date + 1
   - "next [day]" = upcoming occurrence
3. Times in 24-hour HH:MM format
4. Infer defaults:
   - Lunch = 12:00
   - Morning meetings = 9:00
   - Evening events = 18:00
5. Priority:
   - "important", "critical", "urgent" → high/urgent
   - Default: "medium"
6. Categories:
   - Work terms → work
   - Medical terms → health
   - Money terms → finance
   - Social terms → social
   - Learning terms → education
   - Default: "personal"

## Ambiguity Handling
If unclear, set field to null and add to "clarificationQuestions":
- "I see 'lunch next week'—which day works best?"
- "What time should I schedule this?"
- "Should this be recurring?"

Output ONLY valid JSON:"#.to_string()
    }

    pub fn build_extraction_prompt(&self, user_input: &str) -> Vec<ChatMessage> {
        vec![
            ChatMessage {
                role: crate::models::MessageRole::System,
                content: self.extraction_system.clone(),
            },
            ChatMessage {
                role: crate::models::MessageRole::User,
                content: user_input.to_string(),
            },
        ]
    }
}
