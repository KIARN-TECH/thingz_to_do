# **THINGS TO DO**

## **What is this tool?**

This tool is a personal assistant designed to help you manage your daily activities and reminders with ease. Whether you’re a busy professional, student, or anyone who needs help keeping track of tasks, this tool ensures you never miss an important event, meeting, or personal goal.

---

> **Compatibility:**
>
> - **Windows**
> - **Linux**
> - **Mac OS**

---

## **How is it used?**

1. **Add Activities:** Simply input what you need to do, like *"Attend meeting,"* *"Workout,"* *"Catch the Train RB 87,"* or *"Buy groceries."*
2. **Set the Time:** Choose the hour and minute you’d like to be reminded. The tool supports 24-hour format time input.
3. **Get Reminded:** The tool constantly checks the current time and alerts you when it’s time to perform your activity.
4. **View All Activities:** At any time, you can view a list of all your reminders, ensuring you’re always aware of your schedule.
5. **Easy Interaction:** Use simple command-line inputs to add `(y)`, view `(v)`, or modify reminders with ease.

---

## **Benefits**

- **Task Management:** Stay organized by setting reminders for personal and professional tasks.
- **Study Assistant:** Students can set reminders for study sessions, assignments, and exam preparations.
- **Health & Fitness:** Set reminders for workouts, meal plans, or medication schedules.
- **Work Productivity:** Professionals can use it to track meetings, deadlines, and important tasks.
- **Customizable:** Adapt the tool to fit any schedule, whether highly structured or flexible.

---

## **Who Needs This Tool?**

- **Students:** Manage classes, assignments, exams, and extracurricular activities.
- **Busy Professionals:** Keep track of meetings, appointments, and deadlines.
- **Fitness Enthusiasts:** Maintain health goals with workout and meal reminders.
- **Caregivers:** Stay on top of caregiving tasks and medical appointments.
- **Goal-Oriented Individuals:** Build new habits and manage daily activities effectively.

---

## **Pricing Plans**

### **Free Version**

- Basic functionality: Add and view reminders.
- Ad-supported experience.

### **Premium Version**

Includes advanced features such as:

- **Unlimited Reminders:** No limit on reminders.
- **Customizable Notifications:** Advanced settings (recurring reminders, push notifications, etc.).
- **Cross-Device Syncing:** Sync reminders across multiple devices.
- **Reminder Categories:** Organize tasks into categories (work, health, study, etc.).
- **Voice Command Integration:** Manage reminders using voice assistants like Alexa or Google Assistant.

### **Subscription Plans**

| Plan | Price |
|------|------|
| **Monthly** | €4.99/month |
| **Yearly** | €49.99/year *(Save 20%)* |
| **One-Time Purchase** | €29.99 *(Lifetime access to premium features)* |
| **Corporate/Group Plan** | €99/year *(Up to 50 users with premium features)* |
| **Ad-Supported Free Version** | Includes non-intrusive ads, with an option to upgrade to the ad-free premium version. |

---

## **Links to Repository & Website**

- **GitHub Repository:** [https://github.com/KIARN-TECH/thingz_to_do.git](https://github.com/KIARN-TECH/thingz_to_do.git)
- **Share with Colleagues:** [https://kiarn-tech.github.io/thingz_to_do/](https://kiarn-tech.github.io/thingz_to_do/)

---

## **EXPERT SECTION**

### **Thing To Do Tool** *(Technical Overview)*

### **Overview**
This is a simple **CLI-based Reminder Tool** written in **Rust**. It allows users to schedule reminders by providing an activity name and a specific time. The program continuously checks the system time and notifies users when a scheduled reminder is due.

### **Features**

- Accepts user input for reminders (activity name, hour, and minute in 24-hour format).
- Validates input to prevent invalid or past times.
- Displays all saved reminders upon request.
- Checks reminders every minute and notifies the user when it's time.

### **Requirements**

- **Rust** (latest stable version)
- `chrono` crate (for time handling)

### **Installation**

1. Clone the repository:
   ```sh
   git clone https://github.com/KIARN-TECH/thingz_to_do.git
   cd thingz_to_do
   ```
2. Install Rust (if not installed):
   ```sh
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Run the tool:
   ```sh
   cargo run
   ```

### **Usage**

1. Enter the activity name when prompted.
2. Provide the time in 24-hour format (hour and minute separately).
3. The tool validates your input and saves the reminder.
4. Choose to add more reminders, view existing ones, or exit.
5. The tool continuously checks the time and alerts you when a reminder is due.

### **Example Run**
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

### **Future Improvements**

- Implement async scheduling using `tokio`.
- Save and load reminders from a file.
- Add sound or visual notifications.

### **License**
This project is open-source and available under the **MIT License**.

---

This version enhances readability with better formatting, sectioning, and markdown elements. Let me know if you’d like any further refinements!

