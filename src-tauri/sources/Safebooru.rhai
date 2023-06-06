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
    
    `https://safebooru.org/index.php?page=dapi&s=post&q=index&json=1&limit=${limit}&pid=${page}&tags=${tags}`
}

fn parse_search_result(result) {
    const result = parse_json(result);
    let posts = [];
                
    for result in result {
        posts.push(#{
            source: "Safebooru",
            id: result.id,
            tags: #{ "All": result.tags.split(), },
            score: result.score ?? 0,
            rating: switch result.rating {
                "general" => "General",
                "safe" => "Safe",
                "questionable" => "Questionable",
                "explicit" => "Explicit",
                "sensitive" => "Sensitive",
            },
            hash: result.hash,
            resource_url: `https://safebooru.org/images/${result.directory}/${result.image}`,
            preview_url: `https://safebooru.org/samples/${result.directory}/sample_${result.image}`,
        });
    }
    
    posts
}