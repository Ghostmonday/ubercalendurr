import React from 'react';
import { CalendarEvent } from '../../types/event';

interface DayViewProps {
  currentDate: Date;
  events: CalendarEvent[];
  onEventClick?: (event: CalendarEvent) => void;
}

export const DayView: React.FC<DayViewProps> = ({ currentDate, events, onEventClick }) => {
  const dateStr = currentDate.toISOString().split('T')[0];
  const dayEvents = events.filter((e) => e.date === dateStr);
  const hours = Array.from({ length: 24 }, (_, i) => i);
  
  return (
    <div className="bg-gray-800 rounded-xl overflow-hidden">
      <div className="p-4 bg-gray-700 border-b border-gray-600">
        <h2 className="text-xl font-bold">
          {currentDate.toLocaleDateString('en-US', { 
            weekday: 'long', 
            year: 'numeric', 
            month: 'long', 
            day: 'numeric' 
          })}
        </h2>
        <p className="text-sm text-gray-400 mt-1">
          {dayEvents.length} event{dayEvents.length !== 1 ? 's' : ''}
        </p>
      </div>
      
      <div className="overflow-y-auto max-h-[700px]">
        {hours.map((hour) => {
          const hourStr = hour.toString().padStart(2, '0');
          const hourEvents = dayEvents.filter((e) => {
            if (!e.time) return hour === 0;
            const eventHour = parseInt(e.time.split(':')[0]);
            return eventHour === hour;
          });
          
          return (
            <div key={hour} className="flex border-b border-gray-700">
              <div className="w-20 p-3 text-sm text-gray-400 border-r border-gray-700">
                {hourStr}:00
              </div>
              <div className="flex-1 p-2 min-h-[60px]">
                {hourEvents.map((event) => (
                  <div
                    key={event.id}
                    className="mb-2 p-3 rounded cursor-pointer hover:opacity-90 transition"
                    style={{ backgroundColor: event.color || '#6B7280' }}
                    onClick={() => onEventClick?.(event)}
                  >
                    <div className="font-medium">{event.event}</div>
                    {event.time && (
                      <div className="text-xs opacity-75 mt-1">
                        {event.time} - {event.endTime || 'No end time'}
                      </div>
                    )}
                    {event.notes && (
                      <div className="text-xs opacity-75 mt-1">{event.notes}</div>
                    )}
                    <div className="flex gap-2 mt-2">
                      <span className="text-xs px-2 py-0.5 bg-black bg-opacity-20 rounded">
                        {event.category}
                      </span>
                      <span className="text-xs px-2 py-0.5 bg-black bg-opacity-20 rounded">
                        {event.priority}
                      </span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
