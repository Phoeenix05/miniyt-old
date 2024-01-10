import { Routes } from '@angular/router';
import { FeedRoute } from '@routes/feed/feed.route';
import { HistoryRoute } from '@routes/history/history.route';
import { SearchRoute } from '@routes/search/search.route';
import { SettingsRoute } from '@routes/settings/settings.route';
import { VideoRoute } from '@routes/video/video.route';
import { ChangelogRoute } from '../routes/changelog/changelog.route';

export const routes: Routes = [
    { path: '', redirectTo: 'feed', pathMatch: 'full' },
    { path: 'changelog', component: ChangelogRoute },
    { path: 'feed', component: FeedRoute }, // Actual home page
    { path: 'history', component: HistoryRoute },
    { path: 'search/:query', component: SearchRoute },
    { path: 'settings', component: SettingsRoute },
    { path: 'video/:id', component: VideoRoute },
];
