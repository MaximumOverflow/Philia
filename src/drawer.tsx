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
import {ChevronRight, Collections, Dataset as DatasetIcon, Search, Settings, Source as SourceIcon} from "@mui/icons-material";
import {Post, Source} from "./tabs/search";
import {Dataset} from "./tabs/datasets";
import {SavedImage} from "./tabs/images";

interface Props {
    open: boolean,
    sources: Source[]
    datasets: Dataset[],
    images: Map<string, SavedImage>,
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

                <ListItemButton onClick={() => props.set_tab("Images")}>
                    <ListItemIcon><Collections color="primary"/></ListItemIcon>
                    <ListItemText primary="Images" secondary={`Total images: ${props.images.size}`}/>
                </ListItemButton>

                {/*<ListItemButton onClick={() => props.set_tab("Sources")} disabled>*/}
                {/*    <ListItemIcon><SourceIcon color="primary"/></ListItemIcon>*/}
                {/*    <ListItemText primary="Manage sources" secondary={`Installed sources: ${props.sources.length}`}/>*/}
                {/*</ListItemButton>*/}

                <ListItemButton onClick={() => props.set_tab("Settings")}>
                    <ListItemIcon><Settings color="primary"/></ListItemIcon>
                    <ListItemText primary="Settings"/>
                </ListItemButton>
            </List>
        </MuiDrawer>
    );
}