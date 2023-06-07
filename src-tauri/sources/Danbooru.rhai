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
    
    `https://danbooru.donmai.us/posts.json?limit=${limit}&page=${page}&tags=${tags}`
}

fn get_tag_list_url(page, limit, order) {
    const order = switch order.to_int() {
        0 => "id",
        1 => "name",
        2 => "count",
    };
    
    `https://danbooru.donmai.us/tags.json?limit=${limit}&page=${page}&search[order]=${order}`
}

fn parse_search_result(result) {
    const result = parse_json(result);
    let posts = [];
                
    for result in result {
        posts.push(#{
            source: "Danbooru",
            id: result.id,
            tags: #{ 
                "Categorized": #{
                    meta: result.tag_string_meta?.split(),
                    artist: result.tag_string_artist?.split(),
                    general: result.tag_string_general?.split(),
                    character: result.tag_string_character?.split(),
                    copyright: result.tag_string_copyright?.split(),
                } 
            },
            score: result.score,
            rating: switch result.rating {
                "g" => "General",
                "s" => "Safe",
                "q" => "Questionable",
                "e" => "Explicit",
            },
            hash: result.md5,
            resource_url: result.file_url,
            preview_url: result.preview_file_url,
            
            resource_dimensions: [result.image_width ?? 0, result.image_height ?? 0],
            preview_dimensions: [result.media_assets?.variants[0]?.width ?? 0, result.media_assets?.variants[0]?.height ?? 0]
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