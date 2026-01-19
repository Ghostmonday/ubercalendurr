import React, { useState } from 'react';
import { CalendarEvent } from '../../types/event';

interface TerminalInputProps {
  onEventCreated?: (event: CalendarEvent) => void;
}

export const TerminalInput: React.FC<TerminalInputProps> = ({ onEventCreated }) => {
  const [input, setInput] = useState('');
  const [processing, setProcessing] = useState(false);
  const [suggestion, setSuggestion] = useState<string | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!input.trim()) return;

    setProcessing(true);
    setSuggestion(null);

    // Simulate AI processing
    // In production, this would call the DeepSeek API
    setTimeout(() => {
      const newEvent: CalendarEvent = {
        id: crypto.randomUUID(),
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString(),
        date: new Date().toISOString().split('T')[0],
        time: '14:00',
        endTime: '15:00',
        event: input,
        notes: '',
        priority: 'medium',
        category: 'personal',
        color: '#10B981',
        tags: [],
        status: 'confirmed',
        visibility: 'private',
        recurring: null,
        reminder: null,
        location: null,
        metadata: {},
      };

      onEventCreated?.(newEvent);
      setInput('');
      setProcessing(false);
      setSuggestion('Event created successfully!');
      setTimeout(() => setSuggestion(null), 3000);
    }, 1500);
  };

  return (
    <div className="bg-gray-800 rounded-xl p-4 border border-gray-700">
      <div className="flex items-center gap-2 mb-4">
        <span className="text-green-500">›</span>
        <span className="font-mono text-sm text-gray-400">Quick Entry</span>
      </div>

      <form onSubmit={handleSubmit}>
        <div className="relative">
          <textarea
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder="Type an event... (e.g., 'Meeting tomorrow at 2pm')"
            className="w-full h-32 bg-gray-900 border border-gray-700 rounded-lg p-3 font-mono text-sm text-gray-200 placeholder-gray-600 focus:outline-none focus:border-primary-500 resize-none"
            disabled={processing}
          />
          
          <div className="absolute bottom-3 right-3 flex items-center gap-2">
            {processing && (
              <span className="text-xs text-yellow-500 animate-pulse">Processing...</span>
            )}
            <button
              type="submit"
              disabled={!input.trim() || processing}
              className="px-3 py-1 bg-primary-600 hover:bg-primary-700 disabled:bg-gray-700 disabled:text-gray-500 rounded text-sm font-medium transition-colors"
            >
              Enter
            </button>
          </div>
        </div>
      </form>

      {suggestion && (
        <div className="mt-3 p-2 bg-green-900/50 border border-green-700 rounded text-green-400 text-sm">
          {suggestion}
        </div>
      )}

      <div className="mt-4 text-xs text-gray-500">
        <p>Tips:</p>
        <ul className="list-disc list-inside mt-1 space-y-1">
          <li>Type naturally, e.g., "Lunch with John next Tuesday at noon"</li>
          <li>Use JSON for precise entries: {"{event: 'Meeting', date: '2024-01-15'}"}</li>
          <li>Commands: /help, /today, /search</li>
        </ul>
      </div>
    </div>
  );
};
