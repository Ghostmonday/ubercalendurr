import React, { useState, useEffect } from 'react';
import { CalendarGrid } from './components/Calendar/CalendarGrid';
import { TerminalInput } from './components/Terminal/TerminalInput';
import { SettingsPanel } from './components/Settings/SettingsPanel';
import { CalendarEvent } from './types/event';
import { invoke } from '@tauri-apps/api/tauri';

function App() {
  const [currentDate, setCurrentDate] = useState(new Date());
  const [events, setEvents] = useState<CalendarEvent[]>([]);
  const [showSettings, setShowSettings] = useState(false);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadEvents();
  }, [currentDate]);

  const loadEvents = async () => {
    setLoading(true);
    try {
      // TODO: Implement actual IPC call
      const startDate = new Date(currentDate.getFullYear(), currentDate.getMonth(), 1);
      const endDate = new Date(currentDate.getFullYear(), currentDate.getMonth() + 1, 0);
      
      // For demo purposes, add some sample events
      setEvents([
        {
          id: '1',
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          date: new Date().toISOString().split('T')[0],
          time: '09:00',
          endTime: '10:00',
          event: 'Team Standup',
          notes: 'Weekly team meeting',
          priority: 'medium',
          category: 'work',
          color: '#3B82F6',
          tags: ['team'],
          status: 'confirmed',
          visibility: 'private',
          recurring: null,
          reminder: null,
          location: null,
          metadata: {},
        },
        {
          id: '2',
          createdAt: new Date().toISOString(),
          updatedAt: new Date().toISOString(),
          date: new Date().toISOString().split('T')[0],
          time: '12:00',
          endTime: '13:00',
          event: 'Lunch with Sarah',
          notes: '',
          priority: 'low',
          category: 'personal',
          color: '#10B981',
          tags: ['sarah'],
          status: 'confirmed',
          visibility: 'private',
          recurring: null,
          reminder: null,
          location: null,
          metadata: {},
        },
      ]);
    } catch (error) {
      console.error('Failed to load events:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleEventClick = (event: CalendarEvent) => {
    console.log('Event clicked:', event);
  };

  const handleDateClick = (date: Date) => {
    console.log('Date clicked:', date);
  };

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100">
      {/* Header */}
      <header className="bg-gray-800 border-b border-gray-700 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <h1 className="text-xl font-bold">UberCalendurr</h1>
            <span className="text-sm text-gray-500">AI-Powered Calendar</span>
          </div>
          
          <div className="flex items-center gap-4">
            <button
              onClick={() => setShowSettings(!showSettings)}
              className="p-2 hover:bg-gray-700 rounded-lg transition-colors"
            >
              Settings
            </button>
          </div>
        </div>
      </header>

      {/* Main Content */}
      <main className="p-6">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
          {/* Calendar Grid */}
          <div className="lg:col-span-2">
            <CalendarGrid
              onEventClick={handleEventClick}
              onDateClick={handleDateClick}
            />
          </div>

          {/* Terminal Input */}
          <div className="lg:col-span-1">
            <TerminalInput
              onEventCreated={(event) => {
                console.log('Event created:', event);
                loadEvents();
              }}
            />
          </div>
        </div>
      </main>

      {/* Settings Panel */}
      {showSettings && (
        <SettingsPanel onClose={() => setShowSettings(false)} />
      )}
    </div>
  );
}

export default App;
