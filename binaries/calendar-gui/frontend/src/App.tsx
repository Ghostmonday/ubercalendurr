import React, { useState, useEffect } from 'react';
import { CalendarGrid } from './components/Calendar/CalendarGrid';
import { WeekView } from './components/Calendar/WeekView';
import { DayView } from './components/Calendar/DayView';
import { TerminalInput } from './components/Terminal/TerminalInput';
import { SettingsPanel } from './components/Settings/SettingsPanel';
import { EventModal } from './components/Event/EventModal';
import { CalendarEvent } from './types/event';
import { invoke } from '@tauri-apps/api/tauri';

type ViewMode = 'month' | 'week' | 'day';

function App() {
  const [currentDate, setCurrentDate] = useState(new Date());
  const [events, setEvents] = useState<CalendarEvent[]>([]);
  const [showSettings, setShowSettings] = useState(false);
  const [loading, setLoading] = useState(false);
  const [selectedEvent, setSelectedEvent] = useState<CalendarEvent | null>(null);
  const [showEventModal, setShowEventModal] = useState(false);
  const [selectedDate, setSelectedDate] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<ViewMode>('month');

  useEffect(() => {
    loadEvents();
  }, [currentDate]);

  const loadEvents = async () => {
    setLoading(true);
    try {
      const startDate = new Date(currentDate.getFullYear(), currentDate.getMonth(), 1);
      const endDate = new Date(currentDate.getFullYear(), currentDate.getMonth() + 1, 0);
      
      const startDateStr = startDate.toISOString().split('T')[0];
      const endDateStr = endDate.toISOString().split('T')[0];
      
      // Call real Tauri command
      const loadedEvents = await invoke<CalendarEvent[]>('get_events', {
        startDate: startDateStr,
        endDate: endDateStr,
      });
      
      setEvents(loadedEvents);
    } catch (error) {
      console.error('Failed to load events:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleEventClick = (event: CalendarEvent) => {
    setSelectedEvent(event);
    setShowEventModal(true);
  };

  const handleDateClick = (date: Date) => {
    setSelectedDate(date.toISOString().split('T')[0]);
    setSelectedEvent(null);
    setShowEventModal(true);
  };

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100">
      {/* Header */}
      <header className="bg-gray-800 border-b border-gray-700 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <h1 className="text-xl font-bold">UberCalendurr</h1>
            <span className="text-sm text-gray-500">AI-Powered Calendar</span>
            
            {/* View Switcher */}
            <div className="flex gap-2 bg-gray-700 rounded-lg p-1">
              <button
                onClick={() => setViewMode('month')}
                className={`px-3 py-1 rounded transition-colors ${
                  viewMode === 'month' ? 'bg-gray-600' : 'hover:bg-gray-600'
                }`}
              >
                Month
              </button>
              <button
                onClick={() => setViewMode('week')}
                className={`px-3 py-1 rounded transition-colors ${
                  viewMode === 'week' ? 'bg-gray-600' : 'hover:bg-gray-600'
                }`}
              >
                Week
              </button>
              <button
                onClick={() => setViewMode('day')}
                className={`px-3 py-1 rounded transition-colors ${
                  viewMode === 'day' ? 'bg-gray-600' : 'hover:bg-gray-600'
                }`}
              >
                Day
              </button>
            </div>
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
            {viewMode === 'month' && (
              <CalendarGrid
                currentDate={currentDate}
                events={events}
                onEventClick={handleEventClick}
                onDateClick={handleDateClick}
              />
            )}
            {viewMode === 'week' && (
              <WeekView
                currentDate={currentDate}
                events={events}
                onEventClick={handleEventClick}
              />
            )}
            {viewMode === 'day' && (
              <DayView
                currentDate={currentDate}
                events={events}
                onEventClick={handleEventClick}
              />
            )}
          </div>

          {/* Terminal Input */}
          <div className="lg:col-span-1">
            <TerminalInput
              onEventCreated={async (event) => {
                try {
                  await invoke('create_event', { eventData: event });
                  loadEvents();
                } catch (error) {
                  console.error('Failed to create event:', error);
                }
              }}
            />
          </div>
        </div>
      </main>

      {/* Settings Panel */}
      {showSettings && (
        <SettingsPanel onClose={() => setShowSettings(false)} />
      )}

      {/* Event Modal */}
      {showEventModal && (
        <EventModal
          event={selectedEvent || undefined}
          date={selectedDate || undefined}
          onClose={() => {
            setShowEventModal(false);
            setSelectedEvent(null);
            setSelectedDate(null);
          }}
          onSave={loadEvents}
        />
      )}
    </div>
  );
}

export default App;
