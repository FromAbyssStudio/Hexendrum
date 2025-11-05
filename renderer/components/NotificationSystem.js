import React, { useMemo } from 'react';
import { CheckCircle, XCircle, Info, AlertTriangle, X } from 'lucide-react';
import './NotificationSystem.css';

const NotificationSystem = ({ notifications = [], onDismiss }) => {
  const sortedNotifications = useMemo(
    () => [...notifications].sort((a, b) => b.timestamp - a.timestamp),
    [notifications]
  );

  const handleDismiss = (id) => {
    if (onDismiss) {
      onDismiss(id);
    }
  };

  const getIcon = (type) => {
    switch (type) {
      case 'success':
        return <CheckCircle className="notification-icon success" />;
      case 'error':
        return <XCircle className="notification-icon error" />;
      case 'warning':
        return <AlertTriangle className="notification-icon warning" />;
      case 'info':
      default:
        return <Info className="notification-icon info" />;
    }
  };

  const getTypeClass = (type) => `notification-item ${type}`;

  return (
    <div className="notification-system">
      {sortedNotifications.map((notification) => (
        <div
          key={notification.id}
          className={`${getTypeClass(notification.type)} animate-slide-in`}
        >
          <div className="notification-content">
            {getIcon(notification.type)}
            <div className="notification-text">
              <div className="notification-title">{notification.title}</div>
              {notification.message && (
                <div className="notification-message">{notification.message}</div>
              )}
            </div>
          </div>
          <button
            className="notification-close"
            onClick={() => handleDismiss(notification.id)}
            aria-label="Close notification"
          >
            <X size={16} />
          </button>
          <div className="notification-progress" />
        </div>
      ))}
    </div>
  );
};

export default NotificationSystem;
