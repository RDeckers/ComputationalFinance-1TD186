function [] = delta_field( N, M )
    V = compute_field(N, M);
    figure(2)
    mask = [-1 0 1]
    K=15; %strike price
    B = 2*K;
    T=0.5; %time of maturity
    deltaT = T/M;
    r=0.1; %rate of interest
    sigma=0.5; %volatility
    gamma = 1.0;

    S = generate_S(N, B, K);
    delta = diff(V);
    size(delta)
    size(V)
    values = V(end-N+1:end);
    T = fliplr(0:deltaT:T);
    hold on;
    h= pcolor(T, S(1:end-1), delta)
    set(h,'edgecolor','none')
    hold off;
    hcb = colorbar('location','EastOutside');
    caxis([-0.1 0.1])
end

