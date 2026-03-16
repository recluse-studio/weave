use ui_contracts::{SearchAnswer, SearchCitation, SearchResult};

#[must_use]
pub fn synthesize_answer(query: &str, results: &[SearchResult]) -> SearchAnswer {
    let top = results.iter().take(3).collect::<Vec<_>>();
    let citations = top
        .iter()
        .map(|result| SearchCitation {
            label: result.title.clone(),
            target: result.path.clone(),
        })
        .collect::<Vec<_>>();

    let summary = if top.is_empty() {
        format!("No grounded answer was found for \"{query}\" in the current workspace.")
    } else {
        let joined = top
            .iter()
            .map(|result| result.title.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        format!("For \"{query}\", the strongest grounded sources are {joined}.")
    };

    SearchAnswer {
        mode: "best_answer".to_string(),
        summary,
        citations,
    }
}
