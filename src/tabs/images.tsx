import React, {ReactElement, useMemo, useState} from "react";
import {
    Button,
    Dialog, DialogActions,
    DialogContent,
    DialogTitle, IconButton, Stack
} from "@mui/material";
import {removeFile} from "@tauri-apps/api/fs";
import { Settings } from "./settings";
import {PaginatedImageList} from "../components/images";
import {Delete} from "@mui/icons-material";
import {SavedImages} from "../bindings/images";

interface Props {
    settings: Settings,
    images: SavedImages,
    set_images: (images: SavedImages) => void,
}

export function Images(props: Props): ReactElement {
    const [to_delete, set_to_delete] = useState(null as null | string);

    const delete_image = async (path: string) => {
        try {
            await removeFile(path);
            props.set_images(await SavedImages.refresh());
        } 
        finally {
            set_to_delete(null);
        }
    }
    
    const images = useMemo(() => {
        return (
            <PaginatedImageList
                imagesPerPage={128}
                images={props.images.get_all()} 
                fixedPageButtons={true} loadWhenVisible={true}
                actionIcon={image => (
                    <IconButton onClick={() => delete_image(image.file_path)}>
                        <Delete/>
                    </IconButton>
                )}
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
