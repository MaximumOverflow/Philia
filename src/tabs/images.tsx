import React, {ReactElement, useState} from "react";
import {
    Button,
    Dialog, DialogActions,
    DialogContent,
    DialogTitle,
    IconButton,
    ImageList,
    ImageListItem,
    ImageListItemBar,
    Stack
} from "@mui/material";
import {Post} from "./search";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import {Delete, Image} from "@mui/icons-material";
import {removeFile} from "@tauri-apps/api/fs";
import {invoke} from "@tauri-apps/api";

interface Props {
    columns: number,
    images: [string, Post][],
    set_images: (images: [string, Post][]) => void,
}

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
            <ImageList variant="masonry" cols={props.columns} gap={8} style={{padding: ".5em"}}>
                {props.images.map(([path, ]) => (
                    <ImageListItem key={path}>
                        <Stack>
                            <img
                                src={convertFileSrc(path)} alt={path}
                                loading="lazy" className="hover_scale"
                            />
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
                ))}
            </ImageList>
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