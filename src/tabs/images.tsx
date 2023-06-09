import React, {ReactElement, useMemo, useState} from "react";
import {
    Button,
    Dialog, DialogActions,
    DialogContent,
    DialogTitle, Stack
} from "@mui/material";
import {Post} from "./search";
import {removeFile} from "@tauri-apps/api/fs";
import {invoke} from "@tauri-apps/api";
import { Settings } from "./settings";
import {PaginatedImageList} from "../components/images";

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
        const images = [...props.images.values()];

        return (
            <PaginatedImageList 
                images={images} images_per_page={128}
                fixed_page_buttons={true} load_when_visible={true}
            />
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