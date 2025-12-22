#pragma once

#include <wx/wx.h>

class MyApp : public wxApp{
    public:
        bool OnInit() override;
};

class MyFrame : public wxFrame{
    public:
        MyFrame();
    
    private:
        void OnHello(wxCommandEvent& event);
        void OnExit(wxCommandEvent& event);
        void OnAbout(wxCommandEvent& event);
};

enum{
    ID_Hello = 1
};

namespace calculator {
    double add(double a, double b);
    double subtract(double a, double b);
    double multiply(double a, double b);
    double divide(double a, double b);
    void start();
    
    // Complex data structure
    struct ComplexNumber {
        double real;
        double imag;
    };
    
    ComplexNumber addComplex(const ComplexNumber& a, const ComplexNumber& b);
    
    // Async work class
    class HeavyCalculator {
        public:
            static double compute(int iterations);
    };
}