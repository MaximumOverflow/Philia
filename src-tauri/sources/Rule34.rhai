fn get_search_url(page, limit, order, include, exclude) {
    let tags = switch order.to_int() {
        0 => "+sort:id:desc",
        1 => "+sort:id:asc",
        2 => "+sort:score:desc",
        3 => "+sort:score:asc",
        _ => print(`Unknown order ${order}`),
    };
    
    for tag in include { tags = `${tags}+${tag}` }
    for tag in exclude { tags = `${tags}+-${tag}` }
    
    `https://api.rule34.xxx/index.php?page=dapi&s=post&q=index&json=1&limit=${limit}&pid=${page}&tags=${tags}`
}

fn parse_search_result(result) {
    const result = parse_json(result);
    let posts = [];
                
    for result in result {
        posts.push(#{
            source: "Rule34",
            id: result.id,
            tags: #{ "All": result.tags.split(), },
            score: result.score,
            rating: switch result.rating {
                "safe" => "Safe",
                "questionable" => "Questionable",
                "explicit" => "Explicit",
                _ => "General",
            },
            hash: result.hash,
            resource_url: result.file_url,
            preview_url: result.preview_url,
        });
    }
    
    posts
}