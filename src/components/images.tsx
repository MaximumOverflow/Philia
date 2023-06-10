import React, {CSSProperties, MutableRefObject, ReactElement, useEffect, useRef, useState} from "react";
import {
    Button,
    ImageList,
    ImageListItem,
    ImageListItemBar, Paper,
    Stack,
    TextField,
    Tooltip
} from "@mui/material";
import {SavedImage} from "../tabs/images";
import {invoke} from "@tauri-apps/api";

interface ImageProps {
    image: SavedImage,
    actionIcon?: ReactElement,
    preview_size?: number,
    scale_on_hover?: boolean,
    load_when_visible?: boolean,
}

export function SavedImagePreview(props: ImageProps): ReactElement {
    const path = props.image.file_path;
    const preview = props.image.preview_data;

    const [src, set_src] = useState(preview);
    const [stage, set_stage] = useState(0);
    const [visible, ref] = props.load_when_visible
        ? useVisibility<HTMLImageElement>()
        : [false, null];

    useEffect(() => {
        if(stage === 1 && visible)
            invoke<string>("generate_image_preview", {path, size: props.preview_size || 512})
                .then(src => { set_src(src); set_stage(2); })
                .catch(console.error);
    }, [stage, visible]);
    
    return (
        <ImageListItem key={path} className={props.scale_on_hover ? "hover_scale" : ""}>
            <img ref={ref} src={src} alt={path} loading="lazy" onLoad={() => {
                if(stage === 0 && props.load_when_visible) set_stage(1);
            }}/>
            <ImageListItemBar
                title={path.split("/").pop()}
                actionIcon={props.actionIcon}
            />
        </ImageListItem>
    )
}

interface ListProps {
    images: SavedImage[],
    images_per_page: number,
    
    preview_size?: number,
    scale_on_hover?: boolean,
    update_dependencies?: any[]
    load_when_visible?: boolean,
    fixed_page_buttons?: boolean,
    container?: MutableRefObject<any>
    actionIcon?: (image: SavedImage) => ReactElement,
}

const SCROLL_TOP: any = {
    top: 0,
    behavior: "smooth",
};

export function PaginatedImageList(props: ListProps): ReactElement {
    const [page, set_page] = useState(0);
    const [target_page, set_target_page] = useState(0);
    
    useEffect(() => {
        set_page(0);
        set_target_page(0);
    }, [props.images]);
    
    useEffect(() => {
        props.container?.current?.scrollTo(SCROLL_TOP);
    }, [page])

    const offset = page * props.images_per_page;
    const images = [] as ReactElement[];
    for(let i = 0; i < props.images_per_page; i++) {
        const image = props.images[i + offset];
        if(image === undefined) break;
        images.push(
            <SavedImagePreview
                image={image} preview_size={props.preview_size} load_when_visible={props.load_when_visible}
                key={image.file_path} scale_on_hover={props.scale_on_hover}
                actionIcon={props.actionIcon !== undefined ? props.actionIcon(image) : undefined}
            />
        );
    }
    
    const buttons = [] as ReactElement[];
    {
        let middle_added = false;
        const count = Math.ceil(props.images.length / props.images_per_page);
        
        for(let i = 0; i < count; i++) {
            if(count > 6 && i >= 3 && i < count - 3) {
                if(!middle_added) {
                    buttons.push(
                        <Tooltip 
                            key={i}
                            title={
                            <TextField
                                label="Page" value={target_page + 1}
                                type="number" variant="standard"
                                inputProps={{min: 1, max: count}}
                                onChange={e => set_target_page(+e.target.value - 1)}
                            />
                            } 
                            
                            onClose={() => set_page(target_page)}
                        >
                            <Button variant="outlined">...</Button>
                        </Tooltip>
                    );
                    middle_added = true;
                }
                continue;
            }
            
            buttons.push(
                <Button
                    key={i} variant={i === page ? "contained" : "outlined"}
                    onClick={() => set_page(i)}
                >
                    {i + 1}
                </Button>
            )
        }
    }
    
    if(props.fixed_page_buttons) {
        const style: CSSProperties = {
            zIndex: 10,
            left: "50%",
            marginTop: "32px",
            padding: "10px",
            position: "fixed",
            transform: "translateX(-50%)"
        };
        
        if(buttons.length === 1)
            style.display = "none";
        
        return (
            <Stack>
                <Paper style={style}>
                    <Stack direction="row" justifyContent="center" spacing={1}>
                        {buttons}
                    </Stack>
                </Paper>
                <ImageList
                    variant="masonry"
                    cols={4} gap={8} 
                    style={{
                        marginTop: "0",
                        padding: "0 .5em .5em .5em"
                    }}>
                    {images}
                </ImageList>
            </Stack>
        );
    }
    
    return (
        <Stack>
            <ImageList
                variant="masonry"
                cols={4} gap={8} style={{padding: ".5em"}}>
                {images}
            </ImageList>
            <Stack direction="row" justifyContent="center" spacing={1}>
                {buttons}
            </Stack>
        </Stack>
    );
}

function useVisibility<T>(): [boolean, MutableRefObject<T>] {
    const currentElement = useRef(null)
    const [isIntersecting, setIntersecting] = useState(false);

    useEffect(() => {
        const observer = new IntersectionObserver(([entry]) =>
            setIntersecting(entry.isIntersecting)
        );

        // @ts-ignore
        observer.observe(currentElement.current);
        return () => {
            observer.disconnect();
        };
    }, [currentElement]);

    // @ts-ignore
    return [isIntersecting, currentElement];
}