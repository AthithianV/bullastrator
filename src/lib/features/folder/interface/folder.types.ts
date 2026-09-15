import type { ReadQueueModel } from "queue/interface/queue.types";

export interface ReadFolderModel {
  connectionId: string;
  title: string;
  id: string;
}

export interface ReadFolderWithQueuesModel {
  connectionId: string;
  title: string;
  id: string;
  queues: ReadQueueModel[];
}

export interface CreateFolderModel {
  connectionId: string;
  title: string;
}

export interface ReadFolderQueueModel {
  queueId: string;
  folderId: string;
}

export interface FolderTabParams {
  type?: "FOLDER";
  connectionId: string;
  folderId: string;
  folderName: string;
}
