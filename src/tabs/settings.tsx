import React, {ReactElement, useState} from "react";
import {
    Button,
    Collapse,
    List,
    ListItem,
    ListItemButton,
    ListItemIcon,
    ListItemText, ListSubheader, MenuItem, Stack, Switch, TextField, Typography
} from "@mui/material";
import {
    DarkMode,
    DataArray,
    DoNotDisturb,
    ExpandLess,
    ExpandMore, Folder, FormatListNumbered,
    Remove,
    Save,
    Source,
    Tag
} from "@mui/icons-material";
import {downloadDir} from "@tauri-apps/api/path";
import {open} from "@tauri-apps/api/dialog";

interface Props {
    theme: string,
    set_theme: (theme: string) => void,
    sources: string[],
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
        </List>
    );
}

function SearchSettings(props: Props): ReactElement {
    return (
        <List>
            <ListSubheader>Search</ListSubheader>
            <ListItem>
                <ListItemIcon><Source color="primary"/></ListItemIcon>
                <ListItemText primary="Default source"/>
                <TextField 
                    select size="small" variant="standard" 
                    label="Source" defaultValue={props.sources[0]} 
                    style={{minWidth: 250}}
                >
                    {props.sources.map((s, i) => <MenuItem key={i} value={s}>{s}</MenuItem>)}
                </TextField>
            </ListItem>
            
            <ListItem>
                <ListItemIcon><FormatListNumbered color="primary"/></ListItemIcon>
                <ListItemText primary="Tag search result limit"/>
                <TextField type="number" size="small" variant="standard" label="Limit" style={{minWidth: 250}}/>
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
                        <ListItemIcon><Save color="primary"/></ListItemIcon>
                        <ListItemText primary="Save tags"/>
                        <Switch/>
                    </ListItem>

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