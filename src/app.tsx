import {Box, createTheme, CssBaseline, ThemeProvider,} from "@mui/material";
import {Settings, SETTINGS_PLACEHOLDER} from "./tabs/settings";
import {invoke} from "@tauri-apps/api";
import React, {useEffect, useState} from "react";
import {Post, Search, Source} from "./tabs/search";
import {Drawer} from "./drawer";
import {AppBar} from "./appbar";
import {Dataset, Datasets} from "./tabs/datasets";
import {get_saved_images, Images, SavedImage} from "./tabs/images";

const SOURCES = await invoke<Source[]>("get_available_sources");

export default function App() {
    const [tab, set_tab] = useState("Search");
    const [open_drawer, set_drawer_open] = useState(false);

    const [datasets, set_datasets] = useState([] as Dataset[]);
    const [images, set_images] = useState(new Map<string, SavedImage>());
    
    const [settings, set_settings] = useState(SETTINGS_PLACEHOLDER)
    
    useEffect(() => {
        invoke<Settings>("get_settings").then(set_settings);
        invoke<Dataset[]>("get_datasets").then(set_datasets);
        get_saved_images().then(set_images);
    }, []);
    
    useEffect(() => {
        if(settings !== SETTINGS_PLACEHOLDER) {
            invoke("set_settings", {settings}).catch(console.error);
        }
    }, [settings]);

    const theme = createTheme({
        palette: {
            mode: settings.dark_mode ? "dark" : "light",
            primary: {
                "main": settings.accent,
            }
        }
    });
    
    const tabs = {
        "Search": Search({
            sources: SOURCES, 
            datasets, set_datasets,
            columns: settings.search_image_list_columns,
            tag_limit: settings.tag_search_result_limit,
            full_res_search: settings.full_resolution_preview,
            images: images, set_images
        }),

        "Datasets": [
            <Datasets 
                settings={settings} 
                datasets={datasets} set_datasets={set_datasets} 
                all_images={images}
            />,
            null,
        ],

        "Images": [
            <Images 
                images={images} set_images={set_images}
                settings={settings}
            />,
            null,
        ],

        "Settings": [
            <Settings
                sources={SOURCES}
                settings={settings} set_settings={set_settings}
            />,
            null,
        ],
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