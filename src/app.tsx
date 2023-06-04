import {Box, CssBaseline, ThemeProvider,} from "@mui/material";
import React, {useState} from "react";
import {Drawer} from "./drawer";
import {LIGHT_THEME, DARK_THEME} from "./themes";
import {AppBar} from "./appbar";
import {Search, SearchControls} from "./tabs/search";
import {Settings} from "./tabs/settings";

const THEMES: any = {
    "dark": DARK_THEME,
    "light": LIGHT_THEME,
}

export default function App() {
    const [tab, set_tab] = useState("Search");
    const [theme, set_theme] = useState("dark");
    const [open_drawer, set_drawer_open] = useState(false);
    
    const sources = ["Danbooru", "E621", "Rule34"];
    
    let tab_view, tab_controls;
    switch (tab) {
        case "Search": 
            tab_view = <Search/>
            tab_controls = <SearchControls sources={sources}/>
            break;

        case "Settings":
            tab_view = <Settings 
                theme={theme}
                set_theme={set_theme}
                sources={sources}
            />
            break;
    }
    
    return (
        <ThemeProvider theme={THEMES[theme]}>
            <CssBaseline />
            <Box>
                <AppBar tab={tab} controls={tab_controls} set_drawer_open={set_drawer_open}/>
                <Drawer open={open_drawer} set_open={set_drawer_open} set_tab={set_tab}/>
                <Box marginTop="4em">{tab_view}</Box>
            </Box>
        </ThemeProvider>
    );
}