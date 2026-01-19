import React from 'react';
import { CalendarEvent } from '../../types/event';

interface WeekViewProps {
  currentDate: Date;
  events: CalendarEvent[];
  onEventClick?: (event: CalendarEvent) => void;
}

export const WeekView: React.FC<WeekViewProps> = ({ currentDate, events, onEventClick }) => {
  const getWeekDays = () => {
    const start = new Date(currentDate);
    start.setDate(start.getDate() - start.getDay()); // Start of week (Sunday)
    
    return Array.from({ length: 7 }, (_, i) => {
      const date = new Date(start);
      date.setDate(start.getDate() + i);
      return date;
    });
  };
  
  const weekDays = getWeekDays();
  const hours = Array.from({ length: 24 }, (_, i) => i);
  
  return (
    <div className="bg-gray-800 rounded-xl overflow-hidden">
      <div className="grid grid-cols-8 border-b border-gray-700">
        <div className="p-2 text-sm font-medium text-gray-400">Time</div>
        {weekDays.map((day) => (
          <div key={day.toISOString()} className="p-2 text-sm font-medium text-center">
            {day.toLocaleDateString('en-US', { weekday: 'short', month: 'short', day: 'numeric' })}
          </div>
        ))}
      </div>
      
      <div className="overflow-y-auto max-h-[600px]">
        {hours.map((hour) => (
          <div key={hour} className="grid grid-cols-8 border-b border-gray-700">
            <div className="p-2 text-xs text-gray-400">
              {hour.toString().padStart(2, '0')}:00
            </div>
            {weekDays.map((day) => {
              const dateStr = day.toISOString().split('T')[0];
              const hourStr = hour.toString().padStart(2, '0');
              const dayEvents = events.filter((e) => {
                if (e.date !== dateStr) return false;
                if (!e.time) return hour === 0;
                const eventHour = parseInt(e.time.split(':')[0]);
                return eventHour === hour;
              });
              
              return (
                <div
                  key={`${dateStr}-${hour}`}
                  className="p-1 border-l border-gray-700 min-h-[40px]"
                >
                  {dayEvents.map((event) => (
                    <div
                      key={event.id}
                      className="text-xs px-2 py-1 rounded mb-1 cursor-pointer hover:opacity-80"
                      style={{ backgroundColor: event.color || '#6B7280' }}
                      onClick={() => onEventClick?.(event)}
                    >
                      {event.event}
                    </div>
                  ))}
                </div>
              );
            })}
          </div>
        ))}
      </div>
    </div>
  );
};
