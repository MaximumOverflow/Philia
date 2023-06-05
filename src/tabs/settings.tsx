import React, {ReactElement, useRef, useState} from "react";
import {
    Collapse,
    List,
    ListItem,
    ListItemButton,
    ListItemIcon,
    ListItemText, ListSubheader, MenuItem, PaletteMode, Stack, Switch, TextField, Typography
} from "@mui/material";
import {
    DarkMode,
    DataArray,
    DoNotDisturb,
    ExpandLess,
    ExpandMore, Folder, FormatListNumbered, Image,
    Remove,
    Save,
    Source,
    Tag, ViewColumn
} from "@mui/icons-material";
import {downloadDir} from "@tauri-apps/api/path";
import {open} from "@tauri-apps/api/dialog";

interface Props {
    sources: string[],
    
    theme: string,
    set_theme: (theme: PaletteMode) => void,
    
    accent: string,
    set_accent: (accent: string) => void,

    search_columns: number, 
    set_search_columns: (cols: number) => void

    search_tag_limit: number,
    set_search_tag_limit: (tags: number) => void

    full_res_search: boolean,
    set_full_res_search: (full_res: boolean) => void,
}

export function Settings(props: Props): ReactElement {
    return (
        <Stack>
            <GeneralSettings {...props}/>
            <SearchSettings {...props}/>
            <DownloadSettings/>
        </Stack>
    );
}

function GeneralSettings(props: Props): ReactElement {
    const accent = useRef(props.accent);
    
    return (
        <List>
            <ListSubheader>General</ListSubheader>
            <ListItem>
                <ListItemIcon><DarkMode color="primary"/></ListItemIcon>
                <ListItemText primary="Dark mode"/>
                <Switch
                    checked={props.theme === "dark"}
                    onChange={(_, checked) => props.set_theme(checked ? "dark" : "light")}
                />
            </ListItem>

            <ListItem>
                <ListItemIcon><FormatListNumbered color="primary"/></ListItemIcon>
                <ListItemText primary="Accent"/>
                <input 
                    type="color" 
                    value={props.accent} onChange={(e) => accent.current = e.target.value}
                    onBlur={() => props.set_accent(accent.current)}
                    style={{minWidth: 250, border: "none", borderColor: "transparent"}}
                />
            </ListItem>
        </List>
    );
}

function SearchSettings(props: Props): ReactElement {
    return (
        <List>
            <ListSubheader>Search</ListSubheader>
            {/*<ListItem>*/}
            {/*    <ListItemIcon><Source color="primary"/></ListItemIcon>*/}
            {/*    <ListItemText primary="Default source"/>*/}
            {/*    <TextField */}
            {/*        select size="small" variant="standard" */}
            {/*        label="Source" defaultValue={props.sources[0]} */}
            {/*        style={{minWidth: 250}}*/}
            {/*    >*/}
            {/*        {props.sources.map((s, i) => <MenuItem key={i} value={s}>{s}</MenuItem>)}*/}
            {/*    </TextField>*/}
            {/*</ListItem>*/}
            
            <ListItem>
                <ListItemIcon><FormatListNumbered color="primary"/></ListItemIcon>
                <ListItemText primary="Tag search result limit"/>
                <TextField 
                    type="number" inputProps={{min: 10, max: 200}}
                    size="small" variant="standard" label="Limit"
                    value={props.search_tag_limit}
                    onChange={(e) => props.set_search_tag_limit(parseInt(e.target.value) || 10)}
                    style={{minWidth: 250}}
                />
            </ListItem>

            <ListItem>
                <ListItemIcon><ViewColumn color="primary"/></ListItemIcon>
                <ListItemText primary="Image list columns"/>
                <TextField
                    type="number" inputProps={{min: 3, max: 10}}
                    size="small" variant="standard" label="Columns"
                    value={props.search_columns}
                    onChange={(e) => props.set_search_columns(parseInt(e.target.value) || 6)}
                    style={{minWidth: 250}}
                />
            </ListItem>

            <ListItem>
                <ListItemIcon><Image color="primary"/></ListItemIcon>
                <ListItemText primary="Full resolution preview"/>
                <Switch
                    checked={props.full_res_search}
                    onChange={(_, checked) => props.set_full_res_search(checked)}
                />            
            </ListItem>
        </List>
    );
}

const DEFAULT_DOWNLOAD_DIR = await downloadDir();

function DownloadSettings(): ReactElement {
    const [tags, set_tags] = useState(true);
    const [download_dir, set_download_dir] = useState(DEFAULT_DOWNLOAD_DIR);

    return (
        <List>
            <ListSubheader>Downloads</ListSubheader>

            <ListItemButton
                onClick={async () => {
                    try {
                        let dir = await open({
                            directory: true,
                            multiple: false,
                            defaultPath: download_dir,
                            title: "Choose download directory"
                        });

                        if(dir !== null)
                            set_download_dir(dir as string);
                    } finally {}
                }}
            >
                <ListItemIcon><Folder color="primary"/></ListItemIcon>
                <ListItemText primary="Download folder"/>
                <Typography>{download_dir}</Typography>
            </ListItemButton>
            
            <ListItemButton onClick={() => set_tags(!tags)}>
                <ListItemIcon><Tag color="primary"/></ListItemIcon>
                <ListItemText primary="Tags"/>
                {tags ? <ExpandLess color="primary"/> : <ExpandMore color="primary"/>}
            </ListItemButton>
            <Collapse in={tags}>
                <List sx={{pl: 4}}>
                    <ListItem>
                        <ListItemIcon><Remove color="primary"/></ListItemIcon>
                        <ListItemText primary="Remove tag underscores"/>
                        <Switch/>
                    </ListItem>

                    <ListItem>
                        <ListItemIcon><DataArray color="primary"/></ListItemIcon>
                        <ListItemText primary="Escape tag parentheses"/>
                        <Switch/>
                    </ListItem>
                    
                    <ListItem>
                        <ListItemIcon><DoNotDisturb color="primary"/></ListItemIcon>
                        <ListItemText primary="Ignored tag categories"/>
                        <TextField size="small" variant="standard" label="Ignored categories" style={{minWidth: 250}}/>
                    </ListItem>
                </List>
            </Collapse>
        </List>
    );
}