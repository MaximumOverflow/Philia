fn get_search_url(page, limit, order, include, exclude) {
    let tags = switch order.to_int() {
        0 => "+order:id_desc",
        1 => "+order:id_asc",
        2 => "+order:score_desc",
        3 => "+order:score_asc",
        _ => print(`Unknown order ${order}`),
    };
    
    for tag in include { tags = `${tags}+${tag}` }
    for tag in exclude { tags = `${tags}+-${tag}` }
    
    `https://e621.net/posts.json?limit=${limit}&page=${page}&tags=${tags}`
}

fn get_tag_list_url(page, limit, order) {
    const order = switch order.to_int() {
        0 => "date",
        1 => "name",
        2 => "count",
    };
    
    `https://e621.net/tags.json?limit=${limit}&page=${page}&search[order]=${order}`
}

fn parse_search_result(result) {
    const result = parse_json(result);
    let posts = [];
                
    for result in result.posts {
        posts.push(#{
            source: "E621",
            id: result.id,
            tags: #{ "Categorized": result.tags, },
            score: result.score?.total,
            rating: switch result.rating {
                "s" => "Safe",
                "q" => "Questionable",
                "e" => "Explicit",
            },
            hash: result.file?.md5,
            resource_url: result.file?.url,
            preview_url: result.preview?.url,
        });
    }
    
    posts
}

fn parse_tag_list(result) {
    const result = parse_json(result);
    let tags = [];
    
    for result in result {
        tags.push(#{
            id: result.id,
            name: result.name,
            count: result.post_count,
        });
    }
    
    tags
}