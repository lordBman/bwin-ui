#include "calculator.h"
#include <cmath>
#include <thread>
#include <chrono>
 
bool MyApp::OnInit(){
    MyFrame *frame = new MyFrame();
    frame->Show(true);
    return true;
}
 
MyFrame::MyFrame() : wxFrame(nullptr, wxID_ANY, "Hello World"){
    wxMenu *menuFile = new wxMenu;
    menuFile->Append(ID_Hello, "&Hello...\tCtrl-H", "Help string shown in status bar for this menu item");
    menuFile->AppendSeparator();
    menuFile->Append(wxID_EXIT);
 
    wxMenu *menuHelp = new wxMenu;
    menuHelp->Append(wxID_ABOUT);
 
    wxMenuBar *menuBar = new wxMenuBar;
    menuBar->Append(menuFile, "&File");
    menuBar->Append(menuHelp, "&Help");
 
    SetMenuBar( menuBar );
 
    CreateStatusBar();
    SetStatusText("Welcome to wxWidgets!");
 
    Bind(wxEVT_MENU, &MyFrame::OnHello, this, ID_Hello);
    Bind(wxEVT_MENU, &MyFrame::OnAbout, this, wxID_ABOUT);
    Bind(wxEVT_MENU, &MyFrame::OnExit, this, wxID_EXIT);
}
 
void MyFrame::OnExit(wxCommandEvent& event){
    Close(true);
}
 
void MyFrame::OnAbout(wxCommandEvent& event){
    wxMessageBox("This is a wxWidgets Hello World example",
                 "About Hello World", wxOK | wxICON_INFORMATION);
}
 
void MyFrame::OnHello(wxCommandEvent& event){
    wxLogMessage("Hello world from wxWidgets!");
}

namespace calculator {
    double add(double a, double b) {
        return a + b;
    }
    
    double subtract(double a, double b) {
        return a - b;
    }
    
    double multiply(double a, double b) {
        return a * b;
    }
    
    double divide(double a, double b) {
        if (b == 0.0) {
            throw std::runtime_error("Division by zero");
        }
        return a / b;
    }

    void start() {
        MyApp* app = new MyApp();
        wxApp::SetInstance(app);
        app->OnInit();
    }
    
    ComplexNumber addComplex(const ComplexNumber& a, const ComplexNumber& b) {
        return {a.real + b.real, a.imag + b.imag};
    }
    
    double HeavyCalculator::compute(int iterations) {
        double result = 0.0;
        for (int i = 0; i < iterations; ++i) {
            result += std::sin(i * 0.01) * std::cos(i * 0.01);
            // Simulate heavy computation
            std::this_thread::sleep_for(std::chrono::microseconds(10));
        }
        return result;
    }
}