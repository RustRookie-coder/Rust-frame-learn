import type { IEvent, INewEvent } from '@/api/type';
import { defineStore } from 'pinia';
import { fetchEvents, updateEvent } from '@/api/event';
export const useCalendarStore = defineStore('calendar', {
    // arrow function recommended for full type inference
    state: () => ({
        availableCalendars: [
            {
                color: 'error',
                label: 'Personal'
            },
            {
                color: 'primary',
                label: 'Business'
            },
            {
                color: 'warning',
                label: 'Family'
            },
            {
                color: 'success',
                label: 'Holiday'
            },
            {
                color: 'info',
                label: 'Meeting'
            }
        ],
        selectedCalendars: ['Personal', 'Business', 'Family', 'Holiday', 'Meeting']
    }),
    actions: {
        async fetchEvents() {
            return fetchEvents({ calendars: this.selectedCalendars.join(',') });
        },
        async addEvent(event: INewEvent) {
            console.log(event);
            return fetchEvents({ calendars: this.selectedCalendars.join(',') });
        },
        async updateEvent(event: IEvent) {
            return updateEvent(event);
        },
        async removeEvent(eventId: string) {
            return 0;
        }
    }
});