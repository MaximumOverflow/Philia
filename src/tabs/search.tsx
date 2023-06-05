import {
    Autocomplete, Box,
    Button, Checkbox,
    Chip, CircularProgress,
    Dialog,
    DialogActions, DialogContent,
    DialogTitle, IconButton, ImageList, ImageListItem, LinearProgress,
    MenuItem,
    Paper,
    Stack,
    TextField, Typography
} from "@mui/material";
import {Close, Download, Search as SearchIcon} from "@mui/icons-material";
import React, {CSSProperties, ReactElement, useEffect, useState} from "react";
import {invoke} from "@tauri-apps/api";
import {TransformComponent, TransformWrapper} from "react-zoom-pan-pinch";
import {listen} from "@tauri-apps/api/event";

interface Props {
    columns: number,
    sources: string[]
    tag_limit: number,
    full_res_search: boolean,
}

interface Post {
    id: number,
    hash: string,
    score: number,
    rating: string,
    resource_url: string,
    preview_url: string | null,
    tags: {All: string[]} | {Categorized: any},
}

type Order = "Newest" | "Oldest" | "MostLiked" | "LeastLiked";

export function Search(props: Props): ReactElement[] {
    const [selected, set_selected] = useState([] as number[]);
    const [query, set_query] = useState([] as string[]);

    const [order, set_order] = useState("Newest" as Order);
    const [page, set_page] = useState(1);
    const [per_page, set_per_page] = useState(32);

    const [searching, set_searching] = useState(false);
    const [source, set_source] = useState(props.sources[0]);
    const [results, set_results] = useState([] as Post[]);
    const [tags, set_tags] = useState(undefined as (string[] | null | undefined));
    
    useEffect(() => {
        invoke<string[] | null>("get_source_tags", {source})
            .then(result => set_tags(result));
    }, [source])

    const search = async () => {
        try {
            set_searching(true);
            const results = await invoke<Post[]>("search", {
                source: source,
                page: page,
                limit: per_page,
                order: order,
                tags: query,
            });

            set_results(results);
            set_searching(false);
            return results;
        } catch (e) {
            console.error(e)
        }
    };
    
    return [
        <SearchView 
            search={search} 
            searching={searching}
            tags={tags}
            results={results}
            columns={props.columns}
            tag_limit={props.tag_limit}
            full_res_search={props.full_res_search}
            
            query={query} set_query={set_query}
            selected={selected} set_selected={set_selected}
        />,
        <SearchControls 
            selected={selected}
            page={page} set_page={set_page}
            order={order} set_order={set_order}
            per_page={per_page} set_per_page={set_per_page}
            search={search} results={results} searching={searching}
            source={source} set_source={set_source} sources={props.sources}
        />
    ];
}

interface ViewProps {
    columns: number,
    tag_limit: number,
    results: Post[],
    searching: boolean,
    full_res_search: boolean,
    
    query: string[],
    set_query: (query: string[]) => void,

    selected: number[],
    set_selected: (selected: number[]) => void,
    
    tags: string[] | null | undefined,
    search: () => Promise<any[] | undefined>,
}

const IMAGE_VIEW_STYLE: CSSProperties = {maxHeight: "75vh", objectFit: "contain"};
const IMAGE_VIEW_TAG_STYLE: CSSProperties = {display: "inline-block", padding: 2};
const IMAGE_CHECKBOX_STYLE: CSSProperties = {position: "absolute", bottom: 0, right: 0};
const IMAGE_VIEW_TAGS_STYLE: CSSProperties = {maxWidth: "30em", overflow: "auto", maxHeight: "75vh"};

export function SearchView(props: ViewProps): ReactElement {
    const [post_view, set_post_view] = useState(null as Post | null);
    const close_post = () => set_post_view(null);
    
    let post_view_tags: ReactElement | null = null;
    let post_view_media: ReactElement | null = null;
    
    if(post_view !== null) {
        const post = post_view as any;
        if(post.tags["All"] !== undefined) {
            const tags = post.tags["All"] as string[];
            post_view_tags = (
                <Stack sx={IMAGE_VIEW_TAGS_STYLE}>
                    <Typography fontSize={24}>Tags</Typography>
                    {PostViewTags(tags, props)}
                </Stack>
            );
        }
        else if(post.tags["Categorized"] !== undefined) {
            const categories = [] as ReactElement[];
            for(const [key, value] of Object.entries(post.tags["Categorized"])) {
                const tags = value as string[];
                if(tags.length === 0) continue;
                categories.push(
                    <Stack key={key}>
                        <Typography fontSize={24}>{key}</Typography>
                        {PostViewTags(tags, props)}
                    </Stack>
                )
            }
            
            post_view_tags = (
                <Stack sx={IMAGE_VIEW_TAGS_STYLE}>
                    {categories}
                </Stack>
            );
        }
        
        if(post.resource_url.endsWith(".mp4")) {
            post_view_media = (
                <video style={IMAGE_VIEW_STYLE} controls>
                    <source src={post_view?.resource_url} type="video/mp4"/>
                </video>
            );
        }
        else if(post.resource_url.endsWith(".webm")) {
            post_view_media = (
                <video style={IMAGE_VIEW_STYLE} controls>
                    <source src={post_view?.resource_url} type="video/webm"/>
                </video>
            );
        }
        else {
            post_view_media = (
                <TransformWrapper maxScale={3}>
                    <TransformComponent>
                        <img src={post_view?.resource_url} alt={post_view?.id as any} style={IMAGE_VIEW_STYLE}/>
                    </TransformComponent>
                </TransformWrapper>
            );
        }
    }
    
    return (
        <Stack>
            <ImageList variant="masonry" cols={props.columns} gap={8} style={{padding: ".5em"}}>
                {props.results.map((post, i) => PostPreview(i, post, set_post_view, props))}
            </ImageList>

            <Paper
                sx={{
                    left: "50%",
                    width: "90%",
                    marginTop: "2em",
                    position: "fixed",
                    transform: "translate(-50%)",
                    transition: "opacity 0.5s",
                    opacity: 0.5,
                    "&:hover": {opacity: 1 },
                }}
            >
                <Autocomplete
                    multiple freeSolo
                    fullWidth size="small"
                    filterSelectedOptions={true}
                    ChipProps={{color: "primary"}}
                    value={props.query}
                    renderInput={(params) => (
                        <TextField label="Search tags" {...params} variant="filled"/>
                    )}
                    renderTags={(values, props) => values.map((tag, index) => {
                        if(tag.startsWith("-")) {
                            return <Chip label={tag.slice(1)} color="error" {...props({index})}/>
                        }
                        else {
                            return <Chip label={tag} color="success" {...props({index})}/>
                        }
                    })}
                    filterOptions={(options, state) => {
                        let search = state.inputValue;
                        if(search.startsWith("-")) {
                            search = search.slice(1);
                            const results = [] as string[];
                            for(const value of options) {
                                if(results.length >= props.tag_limit) break;
                                if(value.includes(search)) results.push("-" + value);
                            }

                            return results;
                        }
                        else {
                            const results = [] as string[];
                            for(const value of options) {
                                if(results.length >= props.tag_limit) break;
                                if(value.includes(search)) results.push(value);
                            }
                            return results;
                        }
                    }}
                    onKeyDown={async (event) => {
                        const value = (event.target as any).value as string;
                        if (event.key === 'Enter' && value === "" && !props.searching) {
                            await props.search();
                            (event as any).defaultMuiPrevented = true;
                        }
                    }}
                    onChange={(_, values) => props.set_query(values)}
                    options={props.tags || []}
                />
            </Paper>

            <Dialog open={post_view !== null} onClose={close_post} fullWidth={true} classes={{paperFullWidth: "post_view_paper"}}>
                <DialogTitle>
                    <Stack direction="row" alignItems="center">
                        <Typography width="100%" fontSize="22px">
                            Post {post_view?.id}
                        </Typography>
                        <IconButton onClick={close_post}>
                            <Close/>
                        </IconButton>
                    </Stack>
                </DialogTitle>
                <DialogContent style={{maxHeight: "80vh", width: "fit-content"}}>
                    <Stack direction="row" spacing={2}>
                        <Box>{post_view_media}</Box>
                        {post_view_tags}
                    </Stack>
                </DialogContent>
            </Dialog>
        </Stack>
    );
}

function PostPreview(i: number, post: Post, set_post_view: (post: Post) => void, props: ViewProps): ReactElement {
    const toggle_selection = (selected: boolean) => {
        if(selected) {
            const selected = [...props.selected];

            selected.push(i);
            props.set_selected(selected);
            console.log(selected);
        } else {
            const selected = [...props.selected];
            let index = selected.indexOf(i);
            if(index === -1) return;
            selected.splice(index, 1);
            props.set_selected(selected);
            console.log(selected);
        }
    };
    
    const selected = props.selected.includes(i);
    
    return (
        <ImageListItem key={post.id}>
            <Stack>
                <img
                    src={
                        props.full_res_search
                            ? post.resource_url
                            : post.preview_url || post.resource_url
                    }
                    alt={post.id as any}
                    loading="lazy" className="post_list_image"
                    onMouseDown={e => {
                        if(e.ctrlKey) toggle_selection(!selected);
                        else set_post_view(post);
                    }}
                />

                <Checkbox 
                    style={IMAGE_CHECKBOX_STYLE}
                    checked={selected}
                    onChange={(_, checked) => {
                        toggle_selection(checked);
                    }}
                />
            </Stack>
        </ImageListItem>
    )
}

function PostViewTags(tags: string[], props: ViewProps): ReactElement {
    return (
        <ImageList variant="masonry" cols={1}>
            {tags.map(tag => {
                let color: any;
                let onClick: any;
                const neg_tag = "-" + tag;

                let index = props.query.indexOf(tag);
                if(index !== -1) {
                    color = "success"
                    onClick = () => {
                        const query = [...props.query];
                        query[index] = neg_tag;
                        props.set_query(query);
                    };
                }
                else {
                    let index = props.query.indexOf(neg_tag);
                    if(index !== -1) {
                        color = "error";
                        onClick = () => {
                            props.query.splice(index, 1);
                            const query = [...props.query];
                            props.set_query(query);
                        };
                    }
                    else {
                        color = "primary"
                        onClick = () => {
                            const query = [...props.query];
                            query.push(tag);
                            props.set_query(query);
                        };
                    }
                }

                return (
                    <ImageListItem key={tag} style={IMAGE_VIEW_TAG_STYLE}>
                        <Chip label={tag} color={color} onClick={onClick}/>
                    </ImageListItem>
                );
            })}
        </ImageList>
    );
}

interface ControlsProps {
    source: string,
    sources: string[],
    set_source: (source: string) => void,
    
    page: number,
    set_page: (page: number) => void,

    per_page: number,
    set_per_page: (page: number) => void,

    order: Order,
    set_order: (order: Order) => void,

    results: Post[],
    selected: number[],

    searching: boolean,
    search: () => Promise<any[] | undefined>,
}

export function SearchControls(props: ControlsProps): ReactElement {
    const [open_download, set_download_open] = useState(false);
    const close_download = () => set_download_open(false);
    
    return (
        <Stack
            width="100%"
            spacing={2}
            direction="row"
            alignItems="center"
            justifyContent="center"
        >
            <TextField 
                type="number" 
                variant="standard" 
                color="primary" 
                label="Page"
                value={props.page}
                onChange={(e) => props.set_page(parseInt(e.target.value) || 1)}
                style={{width: 80}}
            />
            
            <TextField 
                type="number" 
                variant="standard" 
                color="primary" 
                label="Posts per page" 
                value={props.per_page}
                onChange={(e) => props.set_per_page(parseInt(e.target.value) || 32)}
                style={{width: 100}}
            />
            
            <TextField 
                select 
                label="Source" 
                color="primary"
                variant="standard"
                value={props.source}
                onChange={(e) => props.set_source(e.target.value)}
            >
                {props.sources.map((s, i) => <MenuItem key={i} value={s}>{s}</MenuItem>)}
            </TextField>
            
            <TextField
                select
                label="Sorting"
                color="primary"
                variant="standard"
                value={props.order}
                onChange={(e) => props.set_order(e.target.value as Order)}
            >
                <MenuItem value={"Newest"}>Newest</MenuItem>
                <MenuItem value={"Oldest"}>Oldest</MenuItem>
                <MenuItem value={"MostLiked"}>Most liked</MenuItem>
                <MenuItem value={"LeastLiked"}>Least liked</MenuItem>
            </TextField>

            <Button
                disabled={props.searching}
                color="primary" variant="contained" 
                startIcon={
                    props.searching 
                        ? <CircularProgress size="1em"/> 
                        : <SearchIcon/>
                } 
                onClick={props.search}
            >
                Search
            </Button>
            
            <Button
                disabled={props.searching}
                color="primary" variant="contained" 
                startIcon={<Download/>}
                onClick={() => set_download_open(true)}
            >
                {props.selected.length === 0 ? "Download" : "Download selected"}
            </Button>
            
            <DownloadDialog 
                is_open={open_download} 
                close={close_download} 
                posts={props.results}
                selected={props.selected}
                controls={props}
            />
        </Stack>
    );
}

interface DialogProps {
    is_open: boolean,
    close: () => void,
    posts: Post[],
    selected: number[]
    controls: ControlsProps
}

function DownloadDialog(props: DialogProps): ReactElement {
    const [dataset, set_dataset] = useState("null");
    const [collection, set_collection] = useState("null");

    const [downloading, set_downloading] = useState(false);
    const [download_completion, set_download_completion] = useState(0);

    const download = async () => {
        let unlisten = await listen<number>("download_progress", event => {
            set_download_completion(event.payload)
        });
        
        try {
            set_downloading(true);
            set_download_completion(0);
            
            let posts = props.posts;
            if(props.selected.length) {
                posts = props.selected.map(i => props.posts[i]);
            }
            
            await invoke("download_posts", {
                posts, source: props.controls.source,
                options: {
                    dataset: dataset === "null" ? null : dataset,
                    collection: collection === "null" ? null : collection,
                }
            });
        }
        catch (e) {
            console.error(e);
        }
        finally {
            set_downloading(false);
            props.close();
            unlisten();
        }
    };
    
    if(downloading) {
        return (
            <Dialog open={props.is_open} maxWidth="sm" fullWidth>
                <DialogTitle>Downloading...</DialogTitle>
                <DialogContent>
                    <Stack spacing={2}>
                        <Typography>Progress: {download_completion}%</Typography>
                        <LinearProgress variant="determinate" value={download_completion}/>
                    </Stack>
                </DialogContent>
            </Dialog>
        );
    }
    else {
        return (
            <Dialog open={props.is_open} onClose={props.close}>
                <DialogTitle>Download options</DialogTitle>
                <DialogContent>
                    <Stack spacing={2}>
                        <TextField
                            disabled
                            select fullWidth
                            label="Dataset"
                            color="primary"
                            variant="standard"
                            value={dataset}
                            onChange={e => set_dataset(e.target.value)}
                        >
                            <MenuItem value={"null"}>None</MenuItem>
                            <MenuItem value={"Dataset1"}>Dataset1</MenuItem>
                            <MenuItem value={"Dataset2"}>Dataset2</MenuItem>
                            <MenuItem value={"Dataset3"}>Dataset3</MenuItem>
                            <MenuItem value={"Dataset4"}>Dataset4</MenuItem>
                        </TextField>

                        <TextField
                            disabled
                            select fullWidth
                            label="Collection"
                            color="primary"
                            variant="standard"
                            defaultValue="null"
                            value={collection}
                            onChange={e => set_collection(e.target.value)}
                        >
                            <MenuItem value={"null"}>None</MenuItem>
                            <MenuItem value={"Collection1"}>Collection1</MenuItem>
                            <MenuItem value={"Collection2"}>Collection2</MenuItem>
                            <MenuItem value={"Collection3"}>Collection3</MenuItem>
                            <MenuItem value={"Collection4"}>Collection4</MenuItem>
                        </TextField>
                    </Stack>
                </DialogContent>
                <DialogActions>
                    <Button onClick={download}>Download</Button>
                    <Button onClick={props.close}>Cancel</Button>
                </DialogActions>
            </Dialog>
        );
    }
}