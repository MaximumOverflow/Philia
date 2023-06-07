import React, {CSSProperties, ReactElement, useState} from "react";
import {
    Button,
    Dialog, DialogActions,
    DialogContent,
    DialogTitle,
    IconButton,
    ImageList,
    ImageListItem,
    ImageListItemBar, LinearProgress,
    Stack, Typography
} from "@mui/material";
import {Post} from "./search";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import {Delete} from "@mui/icons-material";
import {removeFile} from "@tauri-apps/api/fs";
import {invoke} from "@tauri-apps/api";
import { Settings } from "./settings";

interface Props {
    settings: Settings,
    images: [string, Post][],
    set_images: (images: [string, Post][]) => void,
}

const IMAGE_PLACEHOLDER_STYLE: CSSProperties = {minHeight: "256px"};

export function Images(props: Props): ReactElement {
    const [to_delete, set_to_delete] = useState(null as null | string);

    const delete_image = async (path: string) => {
        try {
            await removeFile(path);
            props.set_images(await invoke("refresh_images"));
        } 
        finally {
            set_to_delete(null);
        }
    }
    
    return (
        <Stack>
            <ImageList variant="masonry" cols={props.settings.search_image_list_columns} gap={8} style={{padding: ".5em"}}>
                {props.images.map(([path, post]) => {
                    return (
                        <ImageListItem key={path}>
                            <Stack>
                                {
                                    props.settings.image_loading_mode === "Eager"
                                        ?   <img
                                                src={convertFileSrc(path)} alt={path}
                                                loading="eager" className="hover_scale"
                                            />
                                        :   <img
                                                src={convertFileSrc(path)} alt={path}
                                                loading="lazy" className="hover_scale" 
                                                style={IMAGE_PLACEHOLDER_STYLE}
                                                onLoad={e => {
                                                    e.currentTarget.setAttribute("style", "");
                                                    e.currentTarget.onload = null;
                                                }}
                                            />
                                }
                            </Stack>
                            <ImageListItemBar
                                title={path.split("/").pop()}
                                actionIcon={(
                                    <Stack direction="row">
                                        <IconButton onClick={() => set_to_delete(path)}>
                                            <Delete/>
                                        </IconButton>
                                    </Stack>
                                )}
                            />
                        </ImageListItem>
                    );
                })}
            </ImageList>
            
            {/* DELETE DIALOG */}
            <Dialog open={to_delete !== null}>
                <DialogTitle>
                    Confirm deletion
                </DialogTitle>
                <DialogContent>
                    Are you sure you want to delete {to_delete?.split("/").pop()}?
                </DialogContent>
                <DialogActions>
                    <Button onClick={() => delete_image(to_delete!)}>
                        Confirm
                    </Button>
                    <Button onClick={() => set_to_delete(null)}>
                        Cancel
                    </Button>
                </DialogActions>
            </Dialog>
        </Stack>
    )
}