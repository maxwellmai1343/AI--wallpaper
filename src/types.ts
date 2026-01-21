export interface WallpaperRecord {
  id: string;
  prompt: string;
  imagePath: string;
  createdAt: number;
  resolution: {
    width: number;
    height: number;
  };
}

export interface Resolution {
    width: number;
    height: number;
}
