fn get_search_url(page, limit, order, include, exclude) {
    let tags = switch order.to_int() {
        0 => "+sort:id:desc",
        1 => "+sort:id:asc",
        2 => "+sort:score:desc",
        3 => "+sort:score:asc",
        _ => print(`Unknown order ${order}`),
    };
    
    for tag in include { tags = `${tags}+${tag}` }
    for tag in exclude { tags = `${tags}-${tag}` }
    
    `https://gelbooru.com/index.php?page=dapi&s=post&q=index&json=1&limit=${limit}&pid=${page}&tags=${tags}`
}

fn get_tag_list_url(page, limit, order) {
    const order = switch order.to_int() {
        0 => "date",
        1 => "name",
        2 => "count",
    };
    
    `https://gelbooru.com/index.php?page=dapi&s=tag&q=index&json=1&limit=${limit}&pid=${page}&search[order]=${order}`
}

fn parse_search_result(result) {
    const result = parse_json(result);
    let posts = [];
                
    for result in result.post {
        posts.push(#{
            source: "Gelbooru",
            id: result.id,
            tags: #{ "All": result.tags?.split() },
            score: result.score,
            rating: switch result.rating {
                "general" => "General",
                "safe" => "Safe",
                "questionable" => "Questionable",
                "explicit" => "Explicit",
                "sensitive" => "Sensitive",
            },
            hash: result.md5,
            resource_url: result.file_url,
            preview_url: result.preview_url,
        });
    }
    
    posts
}

fn parse_tag_list(result) {
    const result = parse_json(result);
    let tags = [];
    
    for result in result.tag {
        tags.push(#{
            id: result.id,
            name: result.name,
            count: result.count,
        });
    }
    
    tags
}