#include <algorithm>
#include <iostream>
#include <string>
#include <climits>
#include <sstream>
#include <unordered_map>
#include <vector>
using namespace std;

enum class Color {
  Blanco,
  Gris,
  Negro
};

string colorToString(Color c) {
  switch (c) {
    case Color::Blanco: return "Blanco";
    case Color::Gris:   return "Gris";
    case Color::Negro:  return "Negro";
    default:            return "Desconocido";
  }
}

template <typename T>
class Vertice {
private:
  T valor;
  int distancia;
  int tiempo_d;
  int tiempo_f;
  Vertice<T>* previo;
  Color color;

public:
  Vertice(T val): valor(val),
                  distancia(INT_MAX),
                  tiempo_d(0),
                  tiempo_f(0),
                  previo(nullptr),
                  color(Color::Blanco)
  {}

  string toString() const {
    ostringstream oss;

    oss << "Valor: " << valor
        << ", Distancia: " << distancia
        << ", Color: " << colorToString(color)
        << ", Tiempo desc: " << tiempo_d
        << ", Tiempo fin: " << tiempo_f;

    return oss.str();
  }

  void setPrevio(Vertice* v) {
    previo = v;
  }

  void setTiempoD(int d) {
    tiempo_d = d;
  }

  void setTiempoF(int f) {
    tiempo_f = f;
  }

  void setColor(Color c) {
    color = c;
  }

  int getTiempoD() {
    return tiempo_d;
  }

  int getTiempoF() {
    return tiempo_f;
  }

  Color getColor() {
    return color;
  }
};

template <typename T>
class GrafoPesado {
  unordered_map<Vertice<T>*, vector<pair<Vertice<T>*, int>>> adyacencias;

public:
  GrafoPesado(unordered_map<Vertice<T>*, vector<pair<Vertice<T>*, int>>> a): adyacencias(a) {}

  string toString() const {
    ostringstream oss;
    for (const auto& [origen, vecinos] : adyacencias) {
      oss << "Desde " << origen->toString() << " -> ";
      if (vecinos.empty()) {
        oss << "Sin adyacentes";
      } else {
        for (const auto& [destino, peso] : vecinos) {
          oss << "[Dest: " << destino->toString() << ", Peso: " << peso << "] ";
        }
      }
      oss << "\n";
    }
    return oss.str();
  }

  vector<pair<Vertice<T>*, int>> getVecinos(Vertice<T>* v) {
    return adyacencias[v];
  };

  vector<Vertice<T>*> getVertices() {
    vector<Vertice<T>*> vertices;

    for (const pair<Vertice<T>*, vector<pair<Vertice<T>*, int>>> p: adyacencias) {
      vertices.push_back(p.first);
    }

    return vertices;
  }
};


template <typename T>
void dfs(GrafoPesado<T> g, Vertice<T>* u, int* tiempo) {
  (*tiempo) += 1;
  u->setColor(Color::Gris);
  u->setTiempoD(*tiempo);

  for (pair<Vertice<T>*, int> p : g.getVecinos(u)) {
    if (p.first->getColor() == Color::Blanco) {
      p.first->setPrevio(u);
      dfs(g, p.first, tiempo);
    }
  }

  u->setColor(Color::Negro);
  (*tiempo) += 1;
  u->setTiempoF(*tiempo);
}

template <typename T>
void mostrarVertices(vector<Vertice<T>*> vertices) {
  for (Vertice<T>* v : vertices) {
    cout << v->toString() << endl;
  }
}

int main() {
  unordered_map<Vertice<string>*, vector<pair<Vertice<string>*, int>>> a;
  Vertice<string>* v_a = new Vertice<string>("a");
  Vertice<string>* v_b = new Vertice<string>("b");
  Vertice<string>* v_c = new Vertice<string>("c");
  Vertice<string>* v_d = new Vertice<string>("d");
  Vertice<string>* v_e = new Vertice<string>("e");
  Vertice<string>* v_f = new Vertice<string>("f");
  Vertice<string>* v_g = new Vertice<string>("g");
  Vertice<string>* v_h = new Vertice<string>("h");
  Vertice<string>* v_i = new Vertice<string>("i");

  a[v_a] = {{v_b, 3}, {v_c, 2}};
  a[v_b] = {{v_c, 1}, {v_d, 4}};
  a[v_c] = {{v_e, 5}};
  a[v_d] = {{v_e, 6}};
  a[v_e] = {{v_f, 4}};
  a[v_f] = {{v_g, 2}, {v_h, 3}};
  a[v_g] = {{v_i, 1}};
  a[v_h] = {{v_i, 5}};
  a[v_i] = {};

  GrafoPesado g(a);
  vector<Vertice<string>*> vertices = g.getVertices();

  sort(vertices.begin(), vertices.end(), [](Vertice<string>* a, Vertice<string>* b) {
    return a->getTiempoF() < b->getTiempoF();
  });
  
  mostrarVertices(vertices);

  int tiempo = 0;
  dfs(g, v_a, &tiempo);

  cout << "--- DFS ---" << endl;

  sort(vertices.begin(), vertices.end(), [](Vertice<string>* a, Vertice<string>* b) {
    return a->getTiempoF() < b->getTiempoF();
  });

  mostrarVertices(vertices);

  delete v_a;
  delete v_b;
  delete v_c;
  delete v_d;
  delete v_e;
  delete v_f;
  delete v_g;
  delete v_h;
  delete v_i;

  return 0;
}
