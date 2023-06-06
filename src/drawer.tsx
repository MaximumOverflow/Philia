import React, {ReactElement} from "react";
import {
    Divider,
    Drawer as MuiDrawer,
    IconButton,
    List,
    ListItemButton,
    ListItemIcon, ListItemText,
    styled
} from "@mui/material";
import {ChevronRight, Collections, Dataset as DatasetIcon, Search, Settings, Source, Tag} from "@mui/icons-material";
import {Post} from "./tabs/search";
import {Dataset} from "./tabs/datasets";

interface Props {
    open: boolean,
    sources: string[]
    datasets: Dataset[],
    images: [string, Post][],
    set_tab: (tab: string) => void,
    set_open: (open: boolean) => void,
}

const DrawerHeader = styled('div')(({ theme }) => ({
    display: 'flex',
    alignItems: 'center',
    padding: theme.spacing(0, 1),
    // necessary for content to be below app bar
    ...theme.mixins.toolbar,
    justifyContent: 'flex-end',
}));

export function Drawer(props: Props): ReactElement {
    const close = () => props.set_open(false);
    return (
        <MuiDrawer
            anchor="right"
            open={props.open}
            onClose={close}
            sx={{
                width: "auto",
                flexShrink: 0,
                '& .MuiDrawer-paper': {
                    width: "auto",
                    boxSizing: 'border-box',
                },
            }}
        >
            <DrawerHeader style={{paddingRight: "75%"}}>
                <IconButton onClick={close}>
                    <ChevronRight color="primary"/>
                </IconButton>
            </DrawerHeader>
            <Divider/>
            <List>
                <ListItemButton onClick={() => props.set_tab("Search")}>
                    <ListItemIcon><Search color="primary"/></ListItemIcon>
                    <ListItemText primary="Search"/>
                </ListItemButton>
                
                <ListItemButton onClick={() => props.set_tab("Datasets")}>
                    <ListItemIcon><DatasetIcon color="primary"/></ListItemIcon>
                    <ListItemText primary="Datasets" secondary={`Total datasets: ${props.datasets.length}`}/>
                </ListItemButton>

                <ListItemButton onClick={() => props.set_tab("Collections")} disabled>
                    <ListItemIcon><Collections color="primary"/></ListItemIcon>
                    <ListItemText primary="Collections" secondary={`Total images: ${props.images.length}`}/>
                </ListItemButton>

                <ListItemButton onClick={() => props.set_tab("Sources")} disabled>
                    <ListItemIcon><Source color="primary"/></ListItemIcon>
                    <ListItemText primary="Manage sources" secondary={`Installed sources: ${props.sources.length}`}/>
                </ListItemButton>

                <ListItemButton onClick={() => props.set_tab("Settings")}>
                    <ListItemIcon><Settings color="primary"/></ListItemIcon>
                    <ListItemText primary="Settings"/>
                </ListItemButton>
            </List>
        </MuiDrawer>
    );
}