import { lazy } from 'solid-js';
import { render } from 'solid-js/web';
import { Router, Route } from '@solidjs/router';

const App = lazy(() => import('./App'));
const About = lazy(() => import('./About'));
const NotFound = lazy(() => import('./NotFound'));

// /** @typedef { import('solid-js/web').MountableElement } MountableElement */
// render(() => <App />, (/** @type MountableElement */ (document.getElementById('root')) ));


/** @typedef { import('solid-js/web').MountableElement } MountableElement */
render(() =>(
  <Router>
    <Route path="/" component={App} />
    <Route path="/about" component={About} />
    <Route path="*404" component={NotFound} />
  </Router>
), (/** @type MountableElement */ (document.getElementById('root'))) );
