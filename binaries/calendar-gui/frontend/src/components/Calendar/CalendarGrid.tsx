import React, { useMemo } from 'react';
import { CalendarEvent } from '../../types/event';
import { generateCalendarDays } from '../../utils/date';

interface CalendarGridProps {
  onEventClick?: (event: CalendarEvent) => void;
  onDateClick?: (date: Date) => void;
  onEventDrop?: (event: CalendarEvent, newDate: string) => void;
}

export const CalendarGrid: React.FC<CalendarGridProps> = ({
  onEventClick,
  onDateClick,
}) => {
  const currentDate = new Date();
  const calendarDays = useMemo(() => {
    return generateCalendarDays(currentDate.getFullYear(), currentDate.getMonth());
  }, [currentDate]);

  const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];

  return (
    <div className="calendar-grid bg-gray-800 rounded-xl overflow-hidden">
      {/* Month navigator */}
      <div className="flex items-center justify-between px-4 py-3 bg-gray-700">
        <button className="p-2 hover:bg-gray-600 rounded-lg">←</button>
        <h2 className="text-lg font-semibold">
          {currentDate.toLocaleDateString('en-US', { month: 'long', year: 'numeric' })}
        </h2>
        <button className="p-2 hover:bg-gray-600 rounded-lg">→</button>
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
            >
              <div className={`
                text-sm font-medium mb-1
                ${isToday ? 'text-primary-500' : isCurrentMonth ? 'text-gray-200' : 'text-gray-600'}
              `}>
                {day.date.getDate()}
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
