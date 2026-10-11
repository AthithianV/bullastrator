import { id, type WebCommand } from ".";

export const FOLDER_ROUTES: Record<string, WebCommand> = {
  get_all_folders: {
    method: "GET",
    path: (args) => `/folders/connection/${id(args.connectionId, "connectionId")}`,
  },
  get_folder_by_id: {
    method: "GET",
    path: (args) => `/folders/${id(args.id, "id")}`,
  },
  get_queues_for_folder: {
    method: "GET",
    path: (args) => `/folders/${id(args.folderId, "folderId")}/queues`,
  },
  create_folder: {
    method: "POST",
    path: () => "/folders",
    body: (args) => args.data,
  },
  update_folder: {
    method: "PATCH",
    path: (args) => `/folders/${id(args.id, "id")}`,
    body: (args) => ({ title: args.title }),
  },
  delete_folder: {
    method: "DELETE",
    path: (args) => `/folders/${id(args.id, "id")}`,
  },
  toggle_queue_in_folder: {
    method: "POST",
    path: (args) =>
      `/folders/${id(args.folderId, "folderId")}/queues/${id(args.queueId, "queueId")}/toggle`,
  },
  reorder_folder_queues: {
    method: "POST",
    path: (args) => `/folders/${id(args.folderId, "folderId")}/queues/reorder`,
    body: (args) => ({ orderedQueueIds: args.orderedQueueIds }),
  },
};
