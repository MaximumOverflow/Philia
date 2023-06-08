import React, {CSSProperties, ReactElement, useEffect, useMemo, useRef, useState} from "react";
import {
    Accordion, AccordionDetails, AccordionSummary, Autocomplete, Box, Button, Checkbox,
    Dialog, DialogActions, DialogContent,
    DialogTitle,
    IconButton,
    ImageList,
    ImageListItem,
    ImageListItemBar, LinearProgress, List, ListItem, ListItemIcon, ListItemText, MenuItem,
    Stack, Switch, TextField,
    Typography
} from "@mui/material";
import {
    AddBox, Article,
    Check,
    Close, CropDin,
    DataArray,
    Dataset,
    Delete, Deselect,
    DoNotDisturb,
    Edit,
    ExpandMore,
    FileUpload, Image, PhotoSizeSelectLarge,
    Remove, Repeat, SelectAll, Textsms
} from "@mui/icons-material";
import {invoke} from "@tauri-apps/api";
import {convertFileSrc} from "@tauri-apps/api/tauri";
import {open} from "@tauri-apps/api/dialog";
import {Settings} from "./settings";
import {SavedImage} from "./images";
import {PaginatedImageList, SavedImagePreview} from "../components/images";

export interface Dataset {
    name: string,
    images: string[],
    thumbnail: string | null,
    settings: {
        tags: {
            remove_underscores: boolean,
            escape_parentheses: boolean,
            ignore_categories: string[],
            ignore_tags: string[],
        },
        image: {
            apply_letterboxing: boolean,
            target_format: "Png" | "Jpg" | "Bmp" | "Gif" | "Qoi" | "WebP",
            resize: [number, number],
        },
        training: {
            keyword: string,
            repetitions: number,
        }
    }
}

interface Props {
    settings: Settings,
    datasets: Dataset[],
    all_images: Map<string, SavedImage>,
    set_datasets: (datasets: Dataset[]) => void,
}

export function Datasets(props: Props): ReactElement {
    const [edit, set_edit] = useState(-1);
    const [to_delete, set_to_delete] = useState(-1);
    const [exporting, set_exporting] = useState(null as string | null);
    const [columns, set_columns] = useState(Math.floor(window.innerWidth / 270));
    
    useEffect(() => {
        window.addEventListener("resize", () => set_columns(Math.floor(window.innerWidth / 270)));
    }, []);
    
    const new_dataset = async () => {
        const datasets = await invoke<Dataset[]>("new_dataset");
        set_edit(datasets.length - 1);
        props.set_datasets(datasets);
    };
    
    return (
        <Stack padding="0.5em" style={{overflow: "auto", width: "fit-content"}} marginX="auto">
            <ImageList cols={columns} gap={6} style={{overflow: "hidden"}}>
                {props.datasets.map((dataset, i) => DatasetPreview(i, dataset, set_edit, set_to_delete, set_exporting))}
                <ImageListItem style={PREVIEW_STYLE} onClick={new_dataset}>
                    <Stack alignItems="center" justifyItems="center" className="hover_scale">
                        <AddBox style={{fontSize: 256}}/>
                    </Stack>
                </ImageListItem>
            </ImageList>
            
            <EditDatasetDialog
                settings={props.settings}
                index={edit} set_edit={set_edit} all_images={props.all_images}
                datasets={props.datasets} set_datasets={props.set_datasets}
            />
            
            <Dialog open={exporting !== null} maxWidth="sm" fullWidth>
                <DialogTitle>
                    {exporting !== null ? `Exporting ${exporting}...` : "Exported"}
                </DialogTitle>
                <DialogContent>
                    <LinearProgress/>
                </DialogContent>
            </Dialog>

            <Dialog open={to_delete !== -1} maxWidth="sm" fullWidth>
                <DialogTitle>
                    {
                        to_delete !== -1 
                            ? `Are you sure you want to delete "${props.datasets[to_delete].name}" ?` 
                            : "Deleted"
                    }
                </DialogTitle>
                <DialogContent>
                    This operation cannot be undone.
                </DialogContent>
                <DialogActions>
                    <Button onClick={() => set_to_delete(-1)}>Cancel</Button>
                    <Button 
                        onClick={async () => {
                            props.set_datasets(await invoke("del_dataset", {index: to_delete}));
                            set_to_delete(-1);
                        }}
                    >
                        Delete
                    </Button>
                </DialogActions>
            </Dialog>
        </Stack>
    );
}

const DEFAULT_PREVIEW = (
    <Stack alignItems="center" justifyItems="center" className="hover_scale">
        <Dataset style={{fontSize: 256}}/>
    </Stack>
);

const PREVIEW_STYLE: CSSProperties = {
    height: "256px",
    width: "256px"
}

function DatasetPreview(
    i: number, dataset: Dataset,
    set_edit: (index: number) => void,
    set_to_delete: (index: number) => void,
    set_exporting: (name: string | null) => void,
): ReactElement {
    const export_dataset = async (index: number) => {
        try {
            let path = await open({
                directory: true,
                multiple: false,
                title: "Choose export directory"
            });

            if(path !== null) {
                set_exporting(dataset.name);
                await invoke("export_dataset", {index, path});
            }
        } finally {
            set_exporting(null);
        }
    };

    return (
        <ImageListItem key={i} style={PREVIEW_STYLE}>
            {
                dataset.thumbnail === null
                    ? DEFAULT_PREVIEW
                    : <img src={convertFileSrc(dataset.thumbnail)} alt="Missing thumbnail" style={PREVIEW_STYLE}/>
            }
            <ImageListItemBar
                title={dataset.name}
                subtitle={`Images: ${dataset.images.length}`}
                actionIcon={(
                    <Stack direction="row">
                        <IconButton onClick={() => export_dataset(i)}>
                            <FileUpload/>
                        </IconButton>
                        <IconButton onClick={() => set_edit(i)}>
                            <Edit/>
                        </IconButton>
                        <IconButton onClick={() => set_to_delete(i)}>
                            <Delete/>
                        </IconButton>
                    </Stack>
                )}
            />
        </ImageListItem>
    );
}

interface EditProps {
    settings: Settings,
    all_images: Map<string, SavedImage>,
    datasets: Dataset[], index: number,
    set_edit: (index: number) => void,
    set_datasets: (datasets: Dataset[]) => void,
}

const EMPTY_DATASET: Dataset = {
    name: "",
    images: [],
    thumbnail: null,
    settings: {
        tags: {
            remove_underscores: false,
            escape_parentheses: false,
            ignore_categories: [],
            ignore_tags: [],
        },
        image: {
            apply_letterboxing: false,
            target_format: "Png",
            resize: [0, 0],
        },
        training: {
            keyword: "",
            repetitions: 0,
        }
    }
}

function EditDatasetDialog(props: EditProps): ReactElement {
    const dataset = props.datasets[props.index] || EMPTY_DATASET;
    
    const [name, set_name] = useState(dataset.name);
    const [images, set_images] = useState(dataset.images);
    const [thumbnail, set_thumbnail] = useState(dataset.thumbnail);
    
    const [ignore_tags, set_ignore_tags] = useState(dataset.settings.tags.ignore_categories);
    const [ignore_categories, set_ignore_categories] = useState(dataset.settings.tags.ignore_categories);
    const [escape_parentheses, set_escape_parentheses] = useState(dataset.settings.tags.escape_parentheses);
    const [remove_underscores, set_remove_underscores] = useState(dataset.settings.tags.remove_underscores);

    const [resize, set_resize] = useState(dataset.settings.image.resize);
    const [format, set_format] = useState(dataset.settings.image.target_format);
    const [apply_letterboxing, set_apply_letterboxing] = useState(dataset.settings.image.apply_letterboxing);

    const [keyword, set_keyword] = useState(dataset.settings.training.keyword);
    const [repetitions, set_repetitions] = useState(dataset.settings.training.repetitions);
    
    const [manage_images, set_manage_images] = useState(false);
    
    const [tags, set_tags] = useState([] as string[]);
    const [tag_categories, set_tag_categories] = useState([] as string[]);

    const images_to_show = useMemo(() => {
        const to_show = [] as SavedImage[];
        for(const path of images) {
            const image = props.all_images.get(path);
            if(image !== undefined) to_show.push(image);
        }

        return to_show;
    }, [images, props.all_images]);
    
    useEffect(() => {
        set_name(dataset.name);
        set_images(dataset.images);
        set_thumbnail(dataset.thumbnail);
        
        set_ignore_tags(dataset.settings.tags.ignore_tags);
        set_ignore_categories(dataset.settings.tags.ignore_categories);
        set_remove_underscores(dataset.settings.tags.remove_underscores);
        set_escape_parentheses(dataset.settings.tags.escape_parentheses);
        
        set_resize(dataset.settings.image.resize);
        set_format(dataset.settings.image.target_format);
        set_apply_letterboxing(dataset.settings.image.apply_letterboxing);
        
        set_keyword(dataset.settings.training.keyword);
        set_repetitions(dataset.settings.training.repetitions);
    }, [dataset])
    
    useEffect(() => {
        invoke<string[]>("get_image_categories", {imagePaths: images}).then(set_tag_categories);
        invoke<string[]>("get_image_tags", {imagePaths: images, ignoredCategories: ignore_categories}).then(set_tags);
    }, [images]);

    useEffect(() => {
        invoke<string[]>("get_image_tags", {imagePaths: images, ignoredCategories: ignore_categories}).then(set_tags);
    }, [ignore_categories]);

    const close = () => props.set_edit(-1);
    const apply = async () => {
        const datasets = await invoke<Dataset[]>("set_dataset", {
            index: props.index,
            dataset: {
                name,
                images,
                thumbnail,
                settings: {
                    tags: {
                        remove_underscores,
                        escape_parentheses,
                        ignore_categories,
                        ignore_tags,
                    },
                    image: {
                        resize,
                        target_format: format,
                        apply_letterboxing,
                    },
                    training: {
                        keyword,
                        repetitions
                    }
                }
            }
        });

        props.set_datasets(datasets);
        close();
    }
    
    const container = useRef(null as any);
    
    return (
        <Dialog open={props.index !== -1} maxWidth="lg" fullWidth>
            <DialogTitle>
                <Stack direction="row" alignItems="center">
                    <Typography width="100%" fontSize="22px">
                        Edit dataset
                    </Typography>
                    <IconButton onClick={apply}>
                        <Check/>
                    </IconButton>
                    <IconButton onClick={close}>
                        <Close/>
                    </IconButton>
                </Stack>
            </DialogTitle>
            <DialogContent ref={container}>
                <List>
                    <ListItem>
                        <TextField
                            fullWidth
                            type="text" inputProps={{min: 10, max: 200}}
                            size="small" variant="standard" label="Name"
                            value={name}
                            onChange={(e) => set_name(e.target.value)}
                        />
                    </ListItem>

                    <ListItem>
                        <Accordion style={{width: "100%"}}>
                            <AccordionSummary expandIcon={<ExpandMore/>}>
                                <Stack direction="row" alignItems="center" width="100%">
                                    <Typography width="84%">Images</Typography>
                                    <Button 
                                        variant="contained" 
                                        onClick={e => {
                                            set_manage_images(true);
                                            e.stopPropagation();
                                        }}
                                    >
                                        Manage images
                                    </Button>
                                </Stack>
                            </AccordionSummary>
                            <AccordionDetails>
                                <PaginatedImageList
                                    images={images_to_show}
                                    images_per_page={24}
                                    container={container}
                                    update_dependencies={[thumbnail]}
                                    actionIcon={image => (
                                        <IconButton
                                            disabled={thumbnail === image.file_path}
                                            onClick={() => set_thumbnail(image.file_path)}
                                        >
                                            <Image/>
                                        </IconButton>
                                    )}
                                />
                                <ManageImagesDialog
                                    settings={props.settings}
                                    open={manage_images} set_open={set_manage_images}
                                    all_images={props.all_images} images={images} set_images={set_images}
                                />
                            </AccordionDetails>
                        </Accordion>
                    </ListItem>

                    <ListItem>
                        <Accordion style={{width: "100%"}} defaultExpanded>
                            <AccordionSummary expandIcon={<ExpandMore/>}>
                                <Stack direction="row" alignItems="center" width="100%">
                                    <Typography width="84%">Tag settings</Typography>
                                    {/*<Button variant="contained" disabled>*/}
                                    {/*    Bulk edit tags*/}
                                    {/*</Button>*/}
                                </Stack>
                            </AccordionSummary>
                            <AccordionDetails>
                                <List>
                                    <ListItem>
                                        <ListItemIcon><Remove color="primary"/></ListItemIcon>
                                        <ListItemText primary="Remove underscores"/>
                                        <Switch
                                            checked={remove_underscores}
                                            onChange={(_, v) => set_remove_underscores(v)}
                                        />
                                    </ListItem>

                                    <ListItem>
                                        <ListItemIcon><DataArray color="primary"/></ListItemIcon>
                                        <ListItemText primary="Escape parentheses"/>
                                        <Switch
                                            checked={escape_parentheses}
                                            onChange={(_, v) => set_escape_parentheses(v)}
                                        />
                                    </ListItem>

                                    <ListItem>
                                        <ListItemIcon><DoNotDisturb color="primary"/></ListItemIcon>
                                        <ListItemText primary="Ignored categories"/>
                                        <Autocomplete
                                            options={tag_categories}
                                            style={{minWidth: 532}}
                                            value={ignore_categories}
                                            filterSelectedOptions={true}
                                            multiple freeSolo size="small"
                                            ChipProps={{color: "error"}}
                                            renderInput={(params) => (
                                                <TextField 
                                                    placeholder={ignore_categories.length === 0 ? "Categories" : ""}
                                                    variant="standard" {...params} 
                                                />
                                            )}
                                            onChange={(_, v) => set_ignore_categories(v)}
                                        />
                                    </ListItem>

                                    <ListItem>
                                        <ListItemIcon><DoNotDisturb color="primary"/></ListItemIcon>
                                        <ListItemText primary="Ignored tags"/>
                                        <Autocomplete
                                            options={tags}
                                            style={{minWidth: 532}}
                                            filterSelectedOptions={true}
                                            multiple freeSolo size="small"
                                            ChipProps={{color: "error"}}
                                            renderInput={(params) => (
                                                <TextField
                                                    placeholder={ignore_categories.length === 0 ? "Ignored" : ""}
                                                    variant="standard" {...params}
                                                />
                                            )}

                                            value={ignore_tags}
                                            onChange={(_, v) => set_ignore_tags(v)}
                                        />
                                    </ListItem>
                                </List>
                            </AccordionDetails>
                        </Accordion>
                    </ListItem>

                    <ListItem>
                        <Accordion style={{width: "100%"}} defaultExpanded>
                            <AccordionSummary expandIcon={<ExpandMore/>}>
                                <Typography>Image settings</Typography>
                            </AccordionSummary>
                            <AccordionDetails>
                                <List>
                                    <ListItem>
                                        <ListItemIcon><CropDin color="primary"/></ListItemIcon>
                                        <ListItemText primary="Apply letterboxing"/>
                                        <Switch
                                            checked={apply_letterboxing} 
                                            onChange={(_, v) => {
                                                set_apply_letterboxing(v)
                                            }}
                                        />
                                    </ListItem>

                                    <ListItem>
                                        <ListItemIcon><PhotoSizeSelectLarge color="primary"/></ListItemIcon>
                                        <ListItemText primary="Resize"/>
                                        <Stack direction="row" spacing={4} alignItems="center">
                                            <TextField
                                                style={{minWidth: 250}}
                                                size="small" variant="standard" label="Width"
                                                type="number" inputProps={{min: 0, max: 2048, step: 64}}
                                                value={resize[0]} onChange={(e) => {
                                                    set_resize([+e.target.value, resize[1]])
                                                }}
                                            />
                                            <TextField
                                                style={{minWidth: 250}}
                                                size="small" variant="standard" label="Height"
                                                type="number" inputProps={{min: 0, max: 2048, step: 64}}
                                                value={resize[1]} onChange={(e) => {
                                                    set_resize([resize[0], +e.target.value])
                                                }}
                                            />
                                        </Stack>
                                    </ListItem>

                                    <ListItem>
                                        <ListItemIcon><Article color="primary"/></ListItemIcon>
                                        <ListItemText primary="Target image format"/>
                                        <TextField
                                            select
                                            label="Format"
                                            color="primary"
                                            variant="standard"
                                            style={{minWidth: 250}}
                                            value={format} onChange={(e) => {
                                                set_format(e.target.value as any)
                                            }}
                                        >
                                            <MenuItem value={"Png"}>Png</MenuItem>
                                            <MenuItem value={"Jpg"}>Jpeg</MenuItem>
                                            <MenuItem value={"Bmp"}>Bmp</MenuItem>
                                            <MenuItem value={"Gif"}>Gif</MenuItem>
                                            <MenuItem value={"Qoi"}>Qoi</MenuItem>
                                            <MenuItem value={"WebP"}>WebP</MenuItem>
                                        </TextField>
                                    </ListItem>
                                </List>
                            </AccordionDetails>
                        </Accordion>
                    </ListItem>

                    <ListItem>
                        <Accordion style={{width: "100%"}} defaultExpanded>
                            <AccordionSummary expandIcon={<ExpandMore/>}>
                                <Typography>Training settings</Typography>
                            </AccordionSummary>
                            <AccordionDetails>
                                <List>
                                    <ListItem>
                                        <ListItemIcon><Textsms color="primary"/></ListItemIcon>
                                        <ListItemText primary="Keyword"/>
                                        <TextField
                                            style={{minWidth: 532}}
                                            type="text" size="small" variant="standard" label="Keyword" 
                                            value={keyword} onChange={(e) => {
                                                set_keyword(e.target.value)
                                            }}
                                        />
                                    </ListItem>
                                    <ListItem>
                                        <ListItemIcon><Repeat color="primary"/></ListItemIcon>
                                        <ListItemText primary="Repetitions"/>
                                        <TextField
                                            style={{minWidth: 532}}
                                            type="number" inputProps={{min: 1}} 
                                            size="small" variant="standard" label="Repetitions"
                                            value={repetitions} onChange={(e) => {
                                                set_repetitions(+e.target.value)
                                            }}
                                        />
                                    </ListItem>
                                </List>
                            </AccordionDetails>
                        </Accordion>
                    </ListItem>
                </List>
            </DialogContent>
        </Dialog>
    );
}

interface ManageImagesProps {
    settings: Settings,
    all_images: Map<string, SavedImage>,
    
    open: boolean,
    set_open: (open: boolean) => void,

    images: string[],
    set_images: (images: string[]) => void,
}

function ManageImagesDialog(props: ManageImagesProps): ReactElement {
    const [last_update, set_last_update] = useState(0);
    const selected = useRef(new Set<string>());
    
    const close = () => {
        props.set_open(false);
    };

    const apply = () => {
        props.set_images([...selected.current]);
        close();
    };
    
    const select_all = () => {
        selected.current = new Set(props.all_images.keys());
        set_last_update(Date.now());
    };
    
    const select_none = () => {
        selected.current = new Set();
        set_last_update(Date.now());
    };
    
    useEffect(() => {
        selected.current = new Set(props.images.filter(path => props.all_images.has(path)));
        set_last_update(Date.now());
    }, [props.images]);
    
    const toggle_selection = (path: string, value: boolean) => {
        if(!value) selected.current.delete(path);
        else selected.current.add(path);
        set_last_update(Date.now());
    };
    
    const images = useMemo(() => {
        const images = [] as SavedImage[];
        for(const path of props.images) {
            const image = props.all_images.get(path);
            if(image !== undefined) images.push(image);
        }
        
        return images;
    }, [props.images, props.all_images]);
    
    const container = useRef(null as any);
    
    return (
        <Dialog open={props.open} maxWidth="xl" fullWidth>
            <DialogTitle>
                <Stack direction="row" alignItems="center">
                    <Typography width="100%" fontSize="22px">
                        Manage images
                    </Typography>
                    
                    <IconButton onClick={select_all}>
                        <SelectAll/>
                    </IconButton>
                    <IconButton onClick={select_none}>
                        <Deselect/>
                    </IconButton>
                    
                    <Box width={50}/>
                    
                    <IconButton onClick={apply}>
                        <Check/>
                    </IconButton>
                    <IconButton onClick={close}>
                        <Close/>
                    </IconButton>
                </Stack>
            </DialogTitle>
            <DialogContent ref={container}>
                <PaginatedImageList 
                    images={images} 
                    images_per_page={64}
                    container={container} 
                    update_dependencies={[last_update]}
                    actionIcon={image => (
                        <Checkbox
                            checked={selected.current.has(image.file_path)} key="select"
                            onChange={(_, checked) => toggle_selection(image.file_path, checked)}
                        />
                    )}
                />
            </DialogContent>
        </Dialog>
    )
}