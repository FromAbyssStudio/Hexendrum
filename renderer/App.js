import React, { useState, useEffect } from 'react';
import { Routes, Route } from 'react-router-dom';
import Library from './pages/Library';
import Playlists from './pages/Playlists';
import NowPlaying from './pages/NowPlaying';
import Settings from './pages/Settings';
import LoadingScreen from './components/LoadingScreen';
import { AudioProvider, useAudio } from './contexts/AudioContext';
import NavigationRail from './components/layout/NavigationRail';
import HeaderBar from './components/layout/HeaderBar';
import QueuePanel from './components/layout/QueuePanel';
import PlaybackBar from './components/layout/PlaybackBar';
import NotificationSystem from './components/NotificationSystem';
import './styles/App.css';
import './styles/Shell.css';

function App() {
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    console.log('Hexendrum Electron App initialized');
    const timer = setTimeout(() => {
      setIsLoading(false);
    }, 3000);
    return () => clearTimeout(timer);
  }, []);

  if (isLoading) {
    return <LoadingScreen onComplete={() => setIsLoading(false)} />;
  }

  return (
    <AudioProvider>
      <AppShell />
    </AudioProvider>
  );
}

const AppShell = () => {
  const [navCollapsed, setNavCollapsed] = useState(false);
  const [queueVisible, setQueueVisible] = useState(false);
  const { notifications, dismissNotification } = useAudio();

  useEffect(() => {
    if (!window?.electronAPI?.onToggleSidebar) {
      return undefined;
    }

    const handler = () => setNavCollapsed((collapsed) => !collapsed);
    window.electronAPI.onToggleSidebar(handler);

    return () => {
      window.electronAPI.removeAllListeners?.('toggle-sidebar');
    };
  }, []);

  return (
    <div className="shell">
      <NavigationRail collapsed={navCollapsed} onToggle={() => setNavCollapsed(!navCollapsed)} />

      <div className="shell__workspace">
        <HeaderBar onToggleQueue={() => setQueueVisible(!queueVisible)} queueVisible={queueVisible} />

        <div className={`shell__body ${queueVisible ? '' : 'shell__body--queue-hidden'}`}>
          <div className="shell__content">
            <div className="shell__content-scroll">
              <Routes>
                <Route path="/" element={<Library />} />
                <Route path="/library" element={<Library />} />
                <Route path="/playlists" element={<Playlists />} />
                <Route path="/now-playing" element={<NowPlaying />} />
                <Route path="/settings" element={<Settings />} />
              </Routes>
            </div>
          </div>

          <QueuePanel visible={queueVisible} onClose={() => setQueueVisible(false)} />
        </div>
      </div>

      <PlaybackBar onToggleQueue={() => setQueueVisible(!queueVisible)} queueVisible={queueVisible} />
      <NotificationSystem
        notifications={notifications}
        onDismiss={dismissNotification}
      />
    </div>
  );
};

export default App;
