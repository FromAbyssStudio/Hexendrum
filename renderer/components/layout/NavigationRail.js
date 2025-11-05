import React from 'react';
import { NavLink } from 'react-router-dom';
import {
  ChevronLeft,
  ChevronRight,
  Headphones,
  Home,
  Library,
  ListMusic,
  Radio,
  Settings,
  Sparkles,
} from 'lucide-react';
import '../../styles/Shell.css';

const NavigationRail = ({ collapsed, onToggle }) => {
  const navItems = [
    { path: '/', label: 'Discover', icon: Home, exact: true },
    { path: '/library', label: 'Library', icon: Library },
    { path: '/playlists', label: 'Playlists', icon: ListMusic },
    { path: '/now-playing', label: 'Now Playing', icon: Radio },
    { path: '/settings', label: 'Settings', icon: Settings },
  ];

  return (
    <aside className={`nav-rail ${collapsed ? 'nav-rail--collapsed' : ''}`}>
      <div className="nav-rail__brand">
        <button className="nav-rail__logo" onClick={onToggle} aria-label="Toggle navigation">
          <Headphones size={20} />
        </button>
        {!collapsed && (
          <div className="nav-rail__brand-text">
            <span className="nav-rail__brand-title">Hexendrum</span>
            <span className="nav-rail__brand-subtitle">Music Engine</span>
          </div>
        )}
        <button className="nav-rail__collapse" onClick={onToggle} aria-label="Collapse navigation">
          {collapsed ? <ChevronRight size={16} /> : <ChevronLeft size={16} />}
        </button>
      </div>

      <nav className="nav-rail__menu">
        {navItems.map((item) => (
          <NavLink
            key={item.path}
            to={item.path}
            end={item.exact}
            className={({ isActive }) =>
              `nav-rail__item ${isActive ? 'nav-rail__item--active' : ''}`
            }
          >
            <item.icon size={18} />
            {!collapsed && <span>{item.label}</span>}
          </NavLink>
        ))}
      </nav>

      {!collapsed && (
        <div className="nav-rail__footer">
          <div className="nav-rail__pill">
            <Sparkles size={16} />
            <div>
              <span className="nav-rail__pill-title">Daily mix</span>
              <span className="nav-rail__pill-subtitle">Fresh tracks curated for you</span>
            </div>
          </div>
        </div>
      )}
    </aside>
  );
};

export default NavigationRail;
