# **THINGS TO DO**

## **What is this tool?** 

This tool is a personal assistant designed to help you manage your daily activities and reminders with ease. Whether you’re a busy professional, student, or anyone who needs help keeping track of tasks, this tool is here to ensure you never miss an important event, meeting, or personal goal.

> [!NOTE]
>### **COMPATIBILITY:**
 • WINDOWS
 
 • LINUX
 
 • MAC OS 


> [!TIP]
>## **How is it used?**

1.	Add Activities: Simply input `what you need to do`, like "Attend meeting," "Workout," "Catch the Train RB 87," or "Buy groceries."
2.	Set the Time: Choose the `hour` and `minute` you'd like to be reminded. The tool supports 24-hour format time input.
3.	Get Reminded: The tool constantly checks the current time and alerts you when it's time to perform your activity.
4.	View All Activities: At any time, you can view a list of all your reminders, ensuring you’re always aware of your schedule.
5.	Easy Interaction: You interact with the assistant through a simple command-line interface where you `add(y)`, `view(v)`, or `modify` reminders with ease.

> [!IMPORTANT]
> ## **BENEFITS**

 • Task Management: Helps people stay organized by setting reminders for personal and professional tasks.
 
 • Study Assistant: Students can set reminders for study sessions, assignments, and exam preparations.
 
 • Health & Fitness: You can set reminders for daily workouts, meal plans, or taking medication.
 
 • Work Productivity: Professionals can use it to set reminders for meetings, deadlines, and important tasks.
 
 • Customizable: The tool is adaptable to anyone’s schedule, from those with a highly structured day to those with more flexible hours.

> [!IMPORTANT]
>## **Who needs this tool?**

•	Students: Those juggling `classes`, `assignments`,`exams`, `work` and `extracurricular activities` who need to stay on top of deadlines and study times.

•	Busy Professionals: People with a packed schedule who need to track meetings, appointments, and deadlines.

•	Fitness Enthusiasts: Anyone working on health goals, including reminders for workouts, meals, or medications.

•	Caregivers: Individuals looking after others who need reminders for caregiving tasks or medical appointments.

•	Anyone with a Goal-Oriented Mindset: Whether you’re trying to learn something new, build a new habit, or just manage your day more effectively, this tool is designed to keep you on track.

> [!IMPORTANT]
>## **How much does it cost?**

### **Free Version:**

I offer basic functionality to all users for free, allowing you to add reminders and view the list of activities.

### **Premium Version:**

For advanced features, such as:

• Unlimited Reminders: `Remove` any `limitations` on the number of reminders that can be added.

• Customizable Notifications: More `personalized` and `advanced` notification settings (e.g., recurring reminders, push notifications, etc.).

• Cross-Device Syncing: `Sync` reminders `across` devices and platforms (e.g., mobile, desktop).

•	Reminder Categories: `Organize` reminders into different categories (e.g., work, health, study).

• Voice Command Integration: Allow users to add and manage reminders through `voice commands` (integration with virtual assistants like Alexa, Google Assistant, etc.).

### **Subscription Plan:**

Access to premium features. 

1.	Monthly Plan:<sup>€4.99/month.</sup>
2.	Yearly Plan: <sup>€49.99/year (save 20%). </sup> 
3.	One-Time Purchase:<sup>€29.99 for lifetime access to premium features.</sup> 
For users who prefer not to subscribe, we offer a one-time payment option to unlock the premium version of the tool indefinitely.  
4.	Corporate or Group Plans:<sup>€99/year for up to 50 users with premium features. </sup> 
I offer discounted packages for companies, teams, or educational institutions who wish to provide this tool to their employees or students. 
5.	Ad-Supported Free Version:<sup>In the free version,There are non-intrusive ads, Users can choose to upgrade to the ad-free premium version. </sup>

> [!NOTE]
>### Links to the Repository

Repository - [https://github.com/KIARN-TECH/thingz_to_do.git]

Share website to your colleagues - [https://kiarn-tech.github.io/thingz_to_do/]






> [!CAUTION]
> # ***EXPERT SECTION***







## Thing To Do Tool

### Overview
This is a simple CLI-based Reminder Tool written in Rust. It allows users to schedule reminders by providing an activity name and a specific time. The program then continuously checks the system time and notifies the user when a scheduled reminder is due.

> [!NOTE]
>### Features
- Accepts user input for reminders (activity name, hour, and minute in 24-hour format).
- Validates input to prevent invalid or past times.
- Displays all saved reminders upon request.
- Checks reminders every minute and notifies the user when it's time.

> [!CAUTION]
>### Requirements
- Rust (latest stable version)
- `chrono` crate (for time handling)

> [!IMPORTANT]
>### Installation
1. Clone this repository:
   ```sh
   git clone https://github.com/yourusername/rust-reminder-tool.git
   cd rust-reminder-tool
   ```
2. Install Rust (if not installed):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Run the tool:
   ```sh
   cargo run
   ```

> [!TIP]
>### Usage
1. Enter the activity name when prompted.
2. Provide the time in 24-hour format (hour and minute separately).
3. The tool validates your input and saves the reminder.
4. You can choose to add more reminders, view existing ones, or exit.
5. The tool continuously checks the time and alerts you when a reminder is due.

> [!TIP]
>### Example Run
```
My Assistant
What do you want to do:
> Workout
At what time? Write Hour (24-hour format):
> 18
How many minutes past the hour:
> 30
I want to Workout at 18:30.
Do you have something else you would like to do? (y/n) or view reminders (v)?
> v

Your reminders:
1: Workout at 18:30
```

### Future Improvements
- Implement async scheduling using `tokio`.
- Save and load reminders from a file.
- Add sound or visual notifications.

### Links

Repository - [https://github.com/KIARN-TECH/thingz_to_do.git]

### License
This project is open-source and available under the MIT License.

