import type { FolderTabParams } from "../../folder/interface/folder.types";
import type { QueueTabParams } from "queue/interface/queue.types";

export interface ReadTabModel {
  id: string;
  workspaceId: string;
  connectionId: string | null;

  title: string;
  params: QueueTabParams | FolderTabParams;

  isActive: boolean;
  isDirty: boolean;
  isPinned: boolean;
  isPreview: boolean;

  connectionColor?: string;
  connectionLabel?: string;

  rank: number;
}

export interface CreateTabModel {
  connectionId?: string | null;

  title: string;
  params: QueueTabParams | FolderTabParams;

  isPreview?: boolean | null;
}

export interface UpdateTabModel {
  title?: string | null;
  params?: QueueTabParams | null;
  rank?: number | null;

  isActive?: boolean | null;
  isDirty?: boolean | null;
  isPinned?: boolean | null;
  isPreview?: boolean | null;
}
