import React from 'react';
import { NavLink } from 'react-router-dom';
import { 
  Home, 
  Library, 
  List, 
  Play, 
  Settings, 
  ChevronLeft, 
  ChevronRight,
  Music,
  Heart,
  Clock,
  Search
} from 'lucide-react';
import './Sidebar.css';

const Sidebar = ({ collapsed, onToggle }) => {
  const navItems = [
    { path: '/', icon: Home, label: 'Home', exact: true },
    { path: '/library', icon: Library, label: 'Library' },
    { path: '/playlists', icon: List, label: 'Playlists' },
    { path: '/now-playing', icon: Play, label: 'Now Playing' },
    { path: '/settings', icon: Settings, label: 'Settings' }
  ];

  const quickActions = [
    { icon: Music, label: 'Recently Added' },
    { icon: Heart, label: 'Liked Songs' },
    { icon: Clock, label: 'Recently Played' },
    { icon: Search, label: 'Search' }
  ];

  return (
    <aside className={`sidebar ${collapsed ? 'collapsed' : ''}`}>
      {/* Header */}
      <div className="sidebar-header">
        <div className="logo">
          <Music size={24} />
          {!collapsed && <span className="logo-text">Hexendrum</span>}
        </div>
        <button 
          className="sidebar-toggle"
          onClick={onToggle}
          title={collapsed ? 'Expand Sidebar' : 'Collapse Sidebar'}
        >
          {collapsed ? <ChevronRight size={16} /> : <ChevronLeft size={16} />}
        </button>
      </div>

      {/* Navigation */}
      <nav className="sidebar-nav">
        <ul className="nav-list">
          {navItems.map((item) => (
            <li key={item.path}>
              <NavLink
                to={item.path}
                className={({ isActive }) => `nav-link ${isActive ? 'active' : ''}`}
                end={item.exact}
              >
                <item.icon size={20} />
                {!collapsed && <span>{item.label}</span>}
              </NavLink>
            </li>
          ))}
        </ul>
      </nav>

      {/* Quick Actions */}
      {!collapsed && (
        <div className="sidebar-section">
          <h3 className="section-title">Quick Actions</h3>
          <ul className="quick-actions">
            {quickActions.map((action, index) => (
              <li key={index}>
                <button className="quick-action-btn">
                  <action.icon size={18} />
                  <span>{action.label}</span>
                </button>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Library Stats */}
      {!collapsed && (
        <div className="sidebar-section">
          <h3 className="section-title">Library</h3>
          <div className="library-stats">
            <div className="stat-item">
              <span className="stat-label">Tracks</span>
              <span className="stat-value">1,247</span>
            </div>
            <div className="stat-item">
              <span className="stat-label">Albums</span>
              <span className="stat-value">89</span>
            </div>
            <div className="stat-item">
              <span className="stat-label">Artists</span>
              <span className="stat-value">156</span>
            </div>
          </div>
        </div>
      )}

      {/* Footer */}
      <div className="sidebar-footer">
        {!collapsed && (
          <div className="user-info">
            <div className="user-avatar">
              <Music size={16} />
            </div>
            <div className="user-details">
              <span className="user-name">Music Lover</span>
              <span className="user-status">Online</span>
            </div>
          </div>
        )}
      </div>
    </aside>
  );
};

export default Sidebar;
