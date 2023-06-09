import {Stack, Typography} from "@mui/material";
import ReactDOM from "react-dom/client";
import {invoke} from "@tauri-apps/api";
import icon from "./icon.ico";
import React from "react";
import "./style.css"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
        <Stack 
            direction="row" id="splashscreen"
            padding="32px 32px 32px 16px"
            alignItems="center"
        >
            <img src={icon} alt="Icon" width={128}/>
            <Stack justifyContent="center">
                <Typography fontSize={48}>Philia</Typography>
                <Typography fontSize={24}>Initializing...</Typography>
            </Stack>
        </Stack>
    </React.StrictMode>
);

await invoke("initialize");