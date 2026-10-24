CHANGELOG.md

### [0.2.3] - 24-10-2026
- Added a skeleton loading state for Job View, keeping the selected job header visible while details load.
- Added job summary data from the job list for faster Job View header rendering.
- Debounced job detail and log requests to reduce API calls while navigating jobs with the keyboard.
- Updated refresh shortcuts to `Ctrl/Cmd + Shift + R` to avoid conflicting with browser refresh.
- Prevented queue shortcuts from triggering while editing job data or using dialogs.
- Improved Job View and job data editor presentation.

### [0.2.2] - 19-10-2026
- Every Connection will have default folder that will have all queues in the connection. This is not editable.
- Added Short cut support for both folder and Queue Views.
- Search Queue inside Folder View
- Improve UI for Search Command

### [0.2.1] - 17-10-2026
- Show Workers in Folder View.
- Changes Card color for Queues inside Folder View.

## [0.2.0] - 2026-09-16

### Added

- Redis connection management
- BullMQ queue browser
- Queue search
- Job inspection
- Job retry
- Job deletion
- Delayed job promotion
- Queue folders
- Multiple queue tabs
- Bulk job operations
- Folders
