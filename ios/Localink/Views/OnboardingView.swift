import SwiftUI

/// The presentation of the first launch, and of "See the presentation again".
struct OnboardingView: View {
    let onDone: () -> Void
    @State private var page = 0

    private struct Slide: Identifiable {
        let id: Int
        let symbol: String
        let title: LocalizedStringKey
        let text: LocalizedStringKey
    }

    private let slides = [
        Slide(id: 0, symbol: "paperplane.fill", title: "Bienvenue sur LOCALINK",
              text: "Vos fichiers, d'un téléphone à l'autre, près ou loin."),
        Slide(id: 1, symbol: "wifi", title: "À côté",
              text: "Sur le même réseau, les fichiers passent directement d'un téléphone à l'autre, sans serveur."),
        Slide(id: 2, symbol: "qrcode", title: "QR code ou code, au choix",
              text: "Scannez le QR code de l'autre téléphone, ou tapez son code : les deux mènent à la même connexion."),
        Slide(id: 3, symbol: "timer", title: "À distance, 2 minutes",
              text: "Loin l'un de l'autre ? Le fichier passe par le serveur et s'efface automatiquement au bout de 2 minutes."),
        Slide(id: 4, symbol: "eye.slash.fill", title: "Sans compte",
              text: "Aucun compte, aucun traceur, aucune publicité."),
    ]

    var body: some View {
        ZStack {
            AnimatedBackground()
            VStack(spacing: 24) {
                HStack {
                    Spacer()
                    Button("Passer", action: onDone)
                        .foregroundStyle(Theme.muted)
                        .opacity(page < slides.count - 1 ? 1 : 0)
                }
                .padding(.horizontal, 24)
                TabView(selection: $page) {
                    ForEach(slides) { slide in
                        VStack(spacing: 26) {
                            Spacer()
                            ZStack {
                                Circle()
                                    .fill(Theme.gradient.opacity(0.18))
                                    .frame(width: 190, height: 190)
                                    .scaleEffect(page == slide.id ? 1 : 0.6)
                                Circle()
                                    .stroke(Theme.cyan.opacity(0.35), lineWidth: 1.5)
                                    .frame(width: 230, height: 230)
                                    .scaleEffect(page == slide.id ? 1 : 0.4)
                                if slide.id == 0 {
                                    LogoMark(size: 110)
                                } else {
                                    Image(systemName: slide.symbol)
                                        .font(.system(size: 72, weight: .semibold))
                                        .foregroundStyle(Theme.gradient)
                                        .symbolEffect(.bounce, value: page == slide.id)
                                }
                            }
                            .animation(.spring(response: 0.6, dampingFraction: 0.65), value: page)
                            VStack(spacing: 12) {
                                Text(slide.title)
                                    .font(.title.bold())
                                    .multilineTextAlignment(.center)
                                Text(slide.text)
                                    .font(.body)
                                    .foregroundStyle(Theme.muted)
                                    .multilineTextAlignment(.center)
                            }
                            .padding(.horizontal, 30)
                            .offset(y: page == slide.id ? 0 : 30)
                            .opacity(page == slide.id ? 1 : 0)
                            .animation(.spring(response: 0.6, dampingFraction: 0.8).delay(0.1), value: page)
                            Spacer()
                        }
                        .tag(slide.id)
                    }
                }
                .tabViewStyle(.page(indexDisplayMode: .always))
                Button {
                    if page < slides.count - 1 {
                        withAnimation(.spring(response: 0.5, dampingFraction: 0.8)) { page += 1 }
                    } else {
                        onDone()
                    }
                } label: {
                    Text(page < slides.count - 1 ? LocalizedStringKey("Suivant") : LocalizedStringKey("Commencer"))
                        .contentTransition(.opacity)
                }
                .buttonStyle(PrimaryButtonStyle())
                .padding(.horizontal, 24)
                .padding(.bottom, 20)
                .sensoryFeedback(.selection, trigger: page)
            }
            .padding(.top, 12)
        }
    }
}
