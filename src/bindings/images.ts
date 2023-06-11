import {Post} from "../tabs/search";
import {invoke} from "@tauri-apps/api";

export interface SavedImage {
    info: Post,
    file_path: string,
    preview_data: string,
}

export class SavedImages {
    public readonly count: number;
    private readonly images: SavedImage[];
    private readonly images_by_url: Map<string, SavedImage>;
    private readonly images_by_path: Map<string, SavedImage>;

    public constructor(images: SavedImage[]) {
        this.images = images;
        this.images_by_url = new Map<string, SavedImage>();
        this.images_by_path = new Map<string, SavedImage>();
        for(const image of images) {
            this.images_by_url.set(image.file_path, image);
            this.images_by_url.set(image.info.resource_url, image);
        }
        
        this.count = this.images_by_path.size;
    }
    
    public get_all(): SavedImage[] {
        return this.images;
    }
    
    public paths(): IterableIterator<string> {
        return this.images_by_path.keys();
    }
    
    public get_by_resource_url(url: string): SavedImage | undefined {
        return this.images_by_url.get(url);
    }

    public get_by_file_path(path: string): SavedImage | undefined {
        return this.images_by_path.get(path);
    }
    
    public has(path_or_url: string): boolean {
        return this.images_by_path.has(path_or_url) || this.images_by_url.has(path_or_url);
    }

    public static async fetch(): Promise<SavedImages> {
        const images = await invoke<SavedImage[]>("get_images");
        return new SavedImages(images);
    }

    public static async refresh(): Promise<SavedImages> {
        const images = await invoke<SavedImage[]>("refresh_images");
        return new SavedImages(images);
    }
}