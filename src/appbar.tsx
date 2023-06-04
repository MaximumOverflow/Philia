import React, {ReactElement} from "react";
import {
    AppBar as MuiAppBar,
    Box,
    IconButton,
    Toolbar,
    Typography
} from "@mui/material";
import {Menu} from "@mui/icons-material";

interface Props {
    tab: string,
    controls?: ReactElement,
    set_drawer_open: (open: boolean) => void,
}

export function AppBar(props: Props): ReactElement {
    return (
        <MuiAppBar color="inherit">
            <Toolbar style={{padding: 0, paddingLeft: "1em"}}>
                <Typography color="primary" fontWeight="bold" fontSize="1.5em">
                    {props.tab}
                </Typography>
                {props.controls || <Box width="100%"/>}
                <IconButton
                    edge="start"
                    color="inherit"
                    aria-label="open drawer"
                    onClick={() => props.set_drawer_open(true)}
                    sx={{ mr: 2 }}
                >
                    <Menu/>
                </IconButton>
            </Toolbar>
        </MuiAppBar>
    );
}