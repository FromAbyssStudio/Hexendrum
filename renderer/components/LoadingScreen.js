import React, { useState, useEffect } from 'react';
import { Music, Disc, Headphones, Volume2 } from 'lucide-react';
import './LoadingScreen.css';

const LoadingScreen = ({ onComplete }) => {
  const [progress, setProgress] = useState(0);
  const [currentStep, setCurrentStep] = useState(0);
  const [isComplete, setIsComplete] = useState(false);

  const steps = [
    { name: 'Initializing Audio Engine', icon: Volume2, duration: 1000 },
    { name: 'Loading Music Library', icon: Music, duration: 1500 },
    { name: 'Preparing Playlists', icon: Disc, duration: 800 },
    { name: 'Ready to Play', icon: Headphones, duration: 500 }
  ];

  useEffect(() => {
    if (isComplete) return;

    const interval = setInterval(() => {
      setProgress(prev => {
        if (prev >= 100) {
          setIsComplete(true);
          setTimeout(() => onComplete(), 500);
          return 100;
        }
        return prev + 1;
      });
    }, 50);

    return () => clearInterval(interval);
  }, [isComplete, onComplete]);

  useEffect(() => {
    const stepInterval = setInterval(() => {
      setCurrentStep(prev => {
        if (prev < steps.length - 1) {
          return prev + 1;
        }
        return prev;
      });
    }, 1000);

    return () => clearInterval(stepInterval);
  }, []);

  const currentStepData = steps[currentStep];
  const CurrentIcon = currentStepData?.icon || Music;

  return (
    <div className="loading-screen">
      <div className="loading-container">
        {/* Logo and Title */}
        <div className="loading-header animate-fade-in">
          <div className="loading-logo">
            <Music className="logo-icon" />
          </div>
          <h1 className="loading-title">Hexendrum</h1>
          <p className="loading-subtitle">Your Music, Your Way</p>
        </div>

        {/* Progress Bar */}
        <div className="loading-progress animate-fade-in">
          <div className="progress-bar">
            <div 
              className="progress-fill" 
              style={{ width: `${progress}%` }}
            />
          </div>
          <div className="progress-text">{Math.round(progress)}%</div>
        </div>

        {/* Current Step */}
        <div className="loading-step animate-fade-in">
          <CurrentIcon className="step-icon animate-spin" />
          <span className="step-text">{currentStepData?.name || 'Loading...'}</span>
        </div>

        {/* Animated Background Elements */}
        <div className="loading-background">
          <div className="floating-icon icon-1">
            <Music />
          </div>
          <div className="floating-icon icon-2">
            <Disc />
          </div>
          <div className="floating-icon icon-3">
            <Headphones />
          </div>
          <div className="floating-icon icon-4">
            <Volume2 />
          </div>
        </div>

        {/* Loading Tips */}
        <div className="loading-tips animate-fade-in">
          <p className="tip-text">
            💡 Tip: Use Ctrl+O to quickly open music folders
          </p>
        </div>
      </div>
    </div>
  );
};

export default LoadingScreen;

