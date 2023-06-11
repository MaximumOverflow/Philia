import React, {ReactElement, useRef} from "react";
import {
    List,
    ListItem,
    ListItemButton,
    ListItemIcon,
    ListItemText, ListSubheader, MenuItem, Stack, Switch, TextField, Typography
} from "@mui/material";
import {DarkMode, Folder, FormatListNumbered, Image, Update, ViewColumn} from "@mui/icons-material";
import {Source} from "./search"
import {open} from "@tauri-apps/api/dialog";
import {invoke} from "@tauri-apps/api";
import {SavedImages} from "../bindings/images";

export interface Settings {
    dark_mode: boolean,
    accent: string,

    tag_search_result_limit: number,
    search_image_list_columns: number,
    full_resolution_preview: boolean,

    download_folder: string,
    update_branch: "Stable" | "Nightly",
}

export const SETTINGS_PLACEHOLDER: Settings = {
    dark_mode: true,
    accent: "#ffb446",
    tag_search_result_limit: 0,
    search_image_list_columns: 0,
    full_resolution_preview: false,
    download_folder: "",
    update_branch: "Stable",
}

interface Props {
    sources: Source[],
    settings: Settings,
    set_settings: (settings: Settings) => void,
    set_saved_images: (images: SavedImages) => void,
}

const W_250_STYLE = {minWidth: 250};

export function Settings(props: Props): ReactElement {
    return (
        <Stack paddingTop="1em">
            {GeneralSettings(props)}
            {SearchSettings(props)}
            {DownloadSettings(props)}
        </Stack>
    );
}

function GeneralSettings(props: Props): ReactElement {
    const accent = useRef(props.settings.accent);
    
    return (
        <List>
            <ListSubheader>General</ListSubheader>
            <ListItem>
                <ListItemIcon><DarkMode color="primary"/></ListItemIcon>
                <ListItemText primary="Dark mode"/>
                <Switch
                    checked={props.settings.dark_mode}
                    onChange={(_, checked) => {
                        const settings = {...props.settings};
                        settings.dark_mode = checked;
                        props.set_settings(settings);
                    }}
                />
            </ListItem>

            <ListItem>
                <ListItemIcon><FormatListNumbered color="primary"/></ListItemIcon>
                <ListItemText primary="Accent"/>
                <input 
                    type="color"
                    style={{minWidth: 250, border: "none", borderColor: "transparent"}}
                    value={props.settings.accent} onChange={(e) => accent.current = e.target.value}
                    onBlur={() => {
                        const settings = {...props.settings};
                        settings.accent = accent.current;
                        props.set_settings(settings);
                    }}
                />
            </ListItem>

            <ListItem>
                <ListItemIcon><Update color="primary"/></ListItemIcon>
                <ListItemText 
                    primary="Update branch"
                    secondary="This change will be applied at the next restart."
                />
                <TextField
                    select
                    label="Branch"
                    color="primary"
                    variant="standard"
                    value={props.settings.update_branch}
                    onChange={(e) => {
                        const settings = {...props.settings};
                        settings.update_branch = e.target.value as any;
                        props.set_settings(settings);
                    }}
                    style={W_250_STYLE}
                >
                    <MenuItem value={"Stable"}>Stable</MenuItem>
                    <MenuItem value={"Nightly"}>Nightly</MenuItem>
                </TextField>
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
            {/*        style={W_250_STYLE}*/}
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
                    value={props.settings.tag_search_result_limit}
                    onChange={(e) => {
                        const settings = {...props.settings};
                        settings.tag_search_result_limit = parseInt(e.target.value) || 10;
                        props.set_settings(settings);
                    }}
                    style={W_250_STYLE}
                />
            </ListItem>

            <ListItem>
                <ListItemIcon><ViewColumn color="primary"/></ListItemIcon>
                <ListItemText primary="Image list columns"/>
                <TextField
                    type="number" inputProps={{min: 3, max: 10}}
                    size="small" variant="standard" label="Columns"
                    value={props.settings.search_image_list_columns}
                    onChange={(e) => {
                        const settings = {...props.settings};
                        settings.search_image_list_columns = parseInt(e.target.value) || 6;
                        props.set_settings(settings);
                    }}
                    style={W_250_STYLE}
                />
            </ListItem>

            <ListItem>
                <ListItemIcon><Image color="primary"/></ListItemIcon>
                <ListItemText primary="Full resolution preview"/>
                <Switch
                    checked={props.settings.full_resolution_preview}
                    onChange={(_, checked) => {
                        const settings = {...props.settings};
                        settings.full_resolution_preview = checked;
                        props.set_settings(settings);
                    }}
                />            
            </ListItem>
        </List>
    );
}

function DownloadSettings(props: Props): ReactElement {
    return (
        <List>
            <ListSubheader>Downloads</ListSubheader>

            <ListItemButton
                onClick={async () => {
                    try {
                        let dir = await open({
                            directory: true,
                            multiple: false,
                            defaultPath: props.settings.download_folder,
                            title: "Choose download directory"
                        });

                        if(dir !== null) {
                            const settings = {...props.settings};
                            settings.download_folder = dir as string;
                            props.set_settings(settings);
                            await invoke("set_settings", {settings});
                            props.set_saved_images(await SavedImages.refresh());
                        }
                    } finally {}
                }}
            >
                <ListItemIcon><Folder color="primary"/></ListItemIcon>
                <ListItemText primary="Download folder"/>
                <Typography>{props.settings.download_folder}</Typography>
            </ListItemButton>
        </List>
    );
}