import {Box, createTheme, CssBaseline, PaletteMode, ThemeProvider,} from "@mui/material";
import {Settings} from "./tabs/settings";
import {invoke} from "@tauri-apps/api";
import React, {useEffect, useState} from "react";
import {Post, Search} from "./tabs/search";
import {Drawer} from "./drawer";
import {AppBar} from "./appbar";
import {Dataset, Datasets} from "./tabs/datasets";

const SOURCES = await invoke<string[]>("get_available_sources");

export default function App() {
    const [tab, set_tab] = useState("Search");
    const [open_drawer, set_drawer_open] = useState(false);
    
    const [theme_accent, set_theme_accent] = useState("#ffb446");
    const [theme_mode, set_theme_mode] = useState("dark" as PaletteMode);
    
    const [search_columns, set_search_columns] = useState(6);
    const [search_tag_limit, set_search_tag_limit] = useState(10);
    const [full_res_search, set_full_res_search] = useState(false);

    const [datasets, set_datasets] = useState([] as Dataset[]);
    const [images, set_images] = useState([] as [string, Post][]);


    useEffect(() => {
        invoke<Dataset[]>("get_datasets").then(set_datasets);
        invoke<[string, Post][]>("get_images").then(set_images);
    }, []);

    const theme = createTheme({
        palette: {
            mode: theme_mode,
            primary: {
                "main": theme_accent
            }
        }
    });
    
    const tabs = {
        "Search": Search({
            sources: SOURCES, 
            full_res_search, 
            columns: search_columns,
            tag_limit: search_tag_limit,
            set_images
        }),
        
        "Settings": [
            <Settings
                sources={SOURCES}
                theme={theme_mode} set_theme={set_theme_mode} 
                accent={theme_accent} set_accent={set_theme_accent}
                search_columns={search_columns} set_search_columns={set_search_columns}
                full_res_search={full_res_search} set_full_res_search={set_full_res_search}
                search_tag_limit={search_tag_limit} set_search_tag_limit={set_search_tag_limit}
            />,
            null,
        ],
        
        "Datasets": [
            <Datasets datasets={datasets} set_datasets={set_datasets} all_images={images}/>,
            null,
        ]
    };
    
    // @ts-ignore
    const [tab_view, tab_controls] = tabs[tab] || [null, null];
    
    return (
        <ThemeProvider theme={theme}>
            <CssBaseline/>
            <Box>
                <AppBar tab={tab} controls={tab_controls} set_drawer_open={set_drawer_open}/>
                <Drawer 
                    open={open_drawer} set_open={set_drawer_open} set_tab={set_tab} 
                    images={images} datasets={datasets} sources={SOURCES}
                />
                <Box marginTop="3em">{tab_view}</Box>
            </Box>
        </ThemeProvider>
    );
}