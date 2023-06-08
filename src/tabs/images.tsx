import React, {ReactElement, useMemo, useState} from "react";
import {
    Button,
    Dialog, DialogActions,
    DialogContent,
    DialogTitle, IconButton,
    ImageList,
    Stack
} from "@mui/material";
import {Post} from "./search";
import {removeFile} from "@tauri-apps/api/fs";
import {invoke} from "@tauri-apps/api";
import { Settings } from "./settings";
import {SavedImagePreview} from "../components/images";
import {Delete} from "@mui/icons-material";

interface Props {
    settings: Settings,
    images: Map<string, SavedImage>,
    set_images: (images: Map<string, SavedImage>) => void,
}

export interface SavedImage {
    info: Post,
    file_path: string,
    preview_data: string,
}

export function Images(props: Props): ReactElement {
    const [to_delete, set_to_delete] = useState(null as null | string);

    const delete_image = async (path: string) => {
        try {
            await removeFile(path);
            props.set_images(await refresh_saved_images());
        } 
        finally {
            set_to_delete(null);
        }
    }
    
    const images = useMemo(() => {
        const images = [] as ReactElement[];
        for(const image of props.images.values()) {
            images.push(
                <
                    SavedImagePreview
                    image={image} key={image.file_path} 
                    scale_on_hover={true} load_when_visible={true}
                    actionIcon={(
                        <IconButton onClick={() => set_to_delete(image.file_path)}>
                            <Delete/>
                        </IconButton>
                    )}
                />
            );
        }

        return (
            <ImageList
                variant="masonry"
                cols={props.settings.search_image_list_columns}
                gap={8} style={{padding: ".5em"}}>
                {images}
            </ImageList>
        );
    }, [props.images]);
    
    return (
        <Stack>
            {images}
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

export async function get_saved_images(): Promise<Map<string, SavedImage>> {
    const images = await invoke<SavedImage[]>("get_images");
    const map = new Map<string, SavedImage>();
    for(const image of images) map.set(image.file_path, image);
    return map;
}

export async function refresh_saved_images(): Promise<Map<string, SavedImage>> {
    const images = await invoke<SavedImage[]>("refresh_images");
    const map = new Map<string, SavedImage>();
    for(const image of images) map.set(image.file_path, image);
    return map;
}