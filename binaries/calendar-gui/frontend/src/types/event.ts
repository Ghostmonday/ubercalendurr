import { z } from 'zod';

const uuidSchema = z.string().uuid();
const timestampSchema = z.string().datetime();
const prioritySchema = z.enum(['low', 'medium', 'high', 'urgent']);
const categorySchema = z.enum(['work', 'personal', 'health', 'social', 'finance', 'education', 'other']);
const eventStatusSchema = z.enum(['tentative', 'confirmed', 'cancelled', 'completed']);
const visibilitySchema = z.enum(['public', 'private']);

export const calendarEventSchema = z.object({
  id: z.string().uuid(),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
  date: z.string().regex(/^\d{4}-\d{2}-\d{2}$/),
  time: z.string().regex(/^\d{2}:\d{2}$/).optional(),
  endTime: z.string().regex(/^\d{2}:\d{2}$/).optional(),
  event: z.string().min(1).max(500),
  notes: z.string().max(5000).optional(),
  priority: prioritySchema.default('medium'),
  category: categorySchema.default('other'),
  color: z.string().regex(/^#[0-9A-Fa-f]{6}$/).optional(),
  tags: z.array(z.string()).default([]),
  status: eventStatusSchema.default('confirmed'),
  visibility: visibilitySchema.default('private'),
  recurring: z.any().optional(),
  reminder: z.any().optional(),
  location: z.any().optional(),
  metadata: z.record(z.unknown()).default({}),
});

export type CalendarEvent = z.infer<typeof calendarEventSchema>;
export type Priority = z.infer<typeof prioritySchema>;
export type Category = z.infer<typeof categorySchema>;

export const categoryInfo: Record<Category, { name: string; color: string; icon: string }> = {
  work: { name: 'Work', color: '#3B82F6', icon: '💼' },
  personal: { name: 'Personal', color: '#10B981', icon: '🏠' },
  health: { name: 'Health', color: '#EF4444', icon: '❤️' },
  social: { name: 'Social', color: '#8B5CF6', icon: '👥' },
  finance: { name: 'Finance', color: '#F59E0B', icon: '💰' },
  education: { name: 'Education', color: '#6366F1', icon: '📚' },
  other: { name: 'Other', color: '#6B7280', icon: '📌' },
};

export const priorityInfo: Record<Priority, { level: number; label: string; emoji: string }> = {
  low: { level: 0, label: 'Low', emoji: '🟢' },
  medium: { level: 1, label: 'Medium', emoji: '🟡' },
  high: { level: 2, label: 'High', emoji: '🟠' },
  urgent: { level: 3, label: 'Urgent', emoji: '🔴' },
};
