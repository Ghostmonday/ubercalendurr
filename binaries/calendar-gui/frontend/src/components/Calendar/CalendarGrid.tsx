import React, { useMemo, useState, useEffect } from 'react';
import { CalendarEvent } from '../../types/event';
import { generateCalendarDays } from '../../utils/date';

interface CalendarGridProps {
  currentDate: Date;
  events: CalendarEvent[];
  onEventClick?: (event: CalendarEvent) => void;
  onDateClick?: (date: Date) => void;
  onEventDrop?: (event: CalendarEvent, newDate: string) => void;
}

export const CalendarGrid: React.FC<CalendarGridProps> = ({
  currentDate: propCurrentDate,
  events,
  onEventClick,
  onDateClick,
}) => {
  const [currentDate, setCurrentDate] = useState(propCurrentDate);
  const [draggedEvent, setDraggedEvent] = useState<CalendarEvent | null>(null);
  
  // Sync with prop changes
  useEffect(() => {
    setCurrentDate(propCurrentDate);
  }, [propCurrentDate]);
  
  const handleDragStart = (e: React.DragEvent, event: CalendarEvent) => {
    setDraggedEvent(event);
    e.dataTransfer.effectAllowed = 'move';
  };
  
  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
  };
  
  const handleDrop = async (e: React.DragEvent, dateStr: string) => {
    e.preventDefault();
    
    if (!draggedEvent) return;
    
    try {
      const updatedEvent = {
        ...draggedEvent,
        date: dateStr,
        updatedAt: new Date().toISOString(),
      };
      
      await invoke('update_event', {
        eventId: draggedEvent.id,
        eventData: updatedEvent,
      });
      
      setDraggedEvent(null);
      // Trigger reload via parent
      if (onEventClick) {
        onEventClick(updatedEvent as CalendarEvent);
      }
    } catch (error) {
      console.error('Failed to update event:', error);
    }
  };
  
  const calendarDays = useMemo(() => {
    return generateCalendarDays(currentDate.getFullYear(), currentDate.getMonth());
  }, [currentDate]);

  const handlePreviousMonth = () => {
    setCurrentDate(new Date(currentDate.getFullYear(), currentDate.getMonth() - 1, 1));
  };

  const handleNextMonth = () => {
    setCurrentDate(new Date(currentDate.getFullYear(), currentDate.getMonth() + 1, 1));
  };

  const handleToday = () => {
    setCurrentDate(new Date());
  };

  const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

  return (
    <div className="calendar-grid bg-gray-800 rounded-xl overflow-hidden">
      {/* Month navigator */}
      <div className="flex items-center justify-between px-4 py-3 bg-gray-700">
        <button 
          onClick={handlePreviousMonth}
          className="p-2 hover:bg-gray-600 rounded-lg transition-colors"
        >
          ←
        </button>
        <div className="flex items-center gap-3">
          <h2 className="text-lg font-semibold">
            {currentDate.toLocaleDateString('en-US', { month: 'long', year: 'numeric' })}
          </h2>
          <button
            onClick={handleToday}
            className="px-3 py-1 text-sm bg-gray-600 hover:bg-gray-500 rounded transition-colors"
          >
            Today
          </button>
        </div>
        <button 
          onClick={handleNextMonth}
          className="p-2 hover:bg-gray-600 rounded-lg transition-colors"
        >
          →
        </button>
      </div>

      {/* Day headers */}
      <div className="grid grid-cols-7 bg-gray-700 border-b border-gray-600">
        {dayNames.map(day => (
          <div key={day} className="px-2 py-2 text-center text-sm font-medium text-gray-400">
            {day}
          </div>
        ))}
      </div>

      {/* Calendar cells */}
      <div className="grid grid-cols-7">
        {calendarDays.map((day, index) => {
          const dateStr = day.date.toISOString().split('T')[0];
          const isToday = day.isToday;
          const isCurrentMonth = day.isCurrentMonth;
          const dayEvents = events.filter(e => e.date === dateStr);

          return (
            <div
              key={index}
              className={`
                min-h-[100px] p-2 border border-gray-700 cursor-pointer
                ${isCurrentMonth ? 'bg-gray-800' : 'bg-gray-900'}
                ${isToday ? 'ring-2 ring-primary-500' : ''}
                hover:bg-gray-750 transition-colors
              `}
              onClick={() => onDateClick?.(day.date)}
              onDragOver={handleDragOver}
              onDrop={(e) => handleDrop(e, dateStr)}
            >
              <div className={`
                text-sm font-medium mb-1
                ${isToday ? 'text-primary-500' : isCurrentMonth ? 'text-gray-200' : 'text-gray-600'}
              `}>
                {day.date.getDate()}
              </div>
              {dayEvents.length > 0 && (
                <div className="mt-1 space-y-1">
                  {dayEvents.slice(0, 3).map((event) => {
                    const priorityColors = {
                      urgent: 'border-l-4 border-red-500',
                      high: 'border-l-4 border-orange-500',
                      medium: 'border-l-4 border-blue-500',
                      low: 'border-l-4 border-gray-500',
                    };
                    
                    const categoryColors = {
                      work: '#3B82F6',
                      personal: '#10B981',
                      health: '#EF4444',
                      social: '#8B5CF6',
                      finance: '#F59E0B',
                      education: '#06B6D4',
                      other: '#6B7280',
                    };
                    
                    return (
                      <div
                        key={event.id}
                        draggable
                        onDragStart={(e) => handleDragStart(e, event)}
                        className={`text-xs px-1 py-0.5 rounded truncate flex items-center gap-1 ${priorityColors[event.priority]}`}
                        style={{ backgroundColor: event.color || categoryColors[event.category] || '#6B7280' }}
                        onClick={(e) => {
                          e.stopPropagation();
                          onEventClick?.(event);
                        }}
                        title={`${event.time || ''} ${event.event}${event.notes ? '\n' + event.notes : ''}`}
                      >
                        {event.recurring && (
                          <span className="text-xs opacity-75" title="Recurring event">🔄</span>
                        )}
                        {event.priority === 'urgent' && (
                          <span className="text-xs" title="Urgent">!</span>
                        )}
                        <span className="truncate">
                          {event.time && `${event.time} `}
                          {event.event}
                        </span>
                      </div>
                    );
                  })}
                  {dayEvents.length > 3 && (
                    <div className="text-xs text-gray-400 px-1">
                      +{dayEvents.length - 3} more
                    </div>
                  )}
                </div>
              )}
            </div>
          );
        })}
      </div>
    </div>
  );
};
